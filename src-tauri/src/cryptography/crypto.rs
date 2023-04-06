// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate crypto;
extern crate rand;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer, symmetriccipher};
use rand::rngs::OsRng;
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub status: i16,
    pub message: String,
}
use std::fmt::{self, Debug};
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

// Encrypt a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
pub fn encrypt(
    data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    // Create an encryptor instance of the best performing
    // type available for the platform.
    let mut encryptor =
        aes::cbc_encryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    // Each encryption operation encrypts some data from
    // an input buffer into an output buffer. Those buffers
    // must be instances of RefReaderBuffer and RefWriteBuffer
    // (respectively) which keep track of how much data has been
    // read from or written to them.
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    // Each encryption operation will "make progress". "Making progress"
    // is a bit loosely defined, but basically, at the end of each operation
    // either BufferUnderflow or BufferOverflow will be returned (unless
    // there was an error). If the return value is BufferUnderflow, it means
    // that the operation ended while wanting more input data. If the return
    // value is BufferOverflow, it means that the operation ended because it
    // needed more space to output data. As long as the next call to the encryption
    // operation provides the space that was requested (either more input data
    // or more output space), the operation is guaranteed to get closer to
    // completing the full operation - ie: "make progress".
    //
    // Here, we pass the data to encrypt to the enryptor along with a fixed-size
    // output buffer. The 'true' flag indicates that the end of the data that
    // is to be encrypted is included in the input buffer (which is true, since
    // the input data includes all the data to encrypt). After each call, we copy
    // any output data to our result Vec. If we get a BufferOverflow, we keep
    // going in the loop since it means that there is more work to do. We can
    // complete as soon as we get a BufferUnderflow since the encryptor is telling
    // us that it stopped processing data due to not having any more data in the
    // input buffer.
    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;

        // "write_buffer.take_read_buffer().take_remaining()" means:
        // from the writable buffer, create a new readable buffer which
        // contains all data that has been written, and then access all
        // of that data as a slice.
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

// Decrypts a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
//
// This function is very similar to encrypt(), so, please reference
// comments in that function. In non-example code, if desired, it is possible to
// share much of the implementation using closures to hide the operation
// being performed. However, such code would make this example less clear.
pub fn decrypt(
    encrypted_data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor =
        aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}
// //
pub fn get_pw_hash(pw: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    return Argon2::default()
    .hash_password(pw, &salt)
    .unwrap()
    .to_string();
}

pub fn validate_hash(password: &str, hashed_password: &str) -> bool {
    match PasswordHash::new(hashed_password) {
        Err(_) => false,
        Ok(parsed) => match Argon2::default().verify_password(password.as_bytes(), &parsed) {
            Ok(()) => true,
            Err(_) => false,
        },
    }
}



pub fn hash_password(password: &str) -> Result<String,ErrorResponse> {
    let salt = SaltString::generate(&mut OsRng);

    match Argon2::default().hash_password(password.as_bytes(), &salt) {
        Ok(h) => Ok(h.to_string()),
        Err(e) => {
            // log::error!(
            //     target: TARGET_AUTHORIZATION,
            //     "Hashing password got error: \"{}\"",
            //     e.to_string()
            // );
            Err(ErrorResponse {
                status: 500,
                message: e.to_string(),
            })
        }
    }
}
//PROBLEM = 
//https://stackoverflow.com/questions/28108689/how-to-initialize-a-variable-with-a-lifetime
// pub fn get_hashed_pw_iv(pw: &[u8]) -> (PasswordHash, [u8; 16]) {
//     let salt = SaltString::generate(&mut OsRng);

//     // Argon2 with default params (Argon2id v19)
//     // Hash password to PHC string ($argon2id$v=19$...)
//     let password_hash = Argon2::default()
//         .hash_password(pw, &salt)
//         .unwrap()
//         .to_string();
//     // Verify password against PHC string.
//     let parsed_hash = PasswordHash::new(&password_hash).unwrap();

//     //TEST
//     // assert!(Argon2::default()
//     //     .verify_password(password, &parsed_hash)
//     //     .is_ok());

//     let mut iv: [u8; 16] = [0; 16];
//     let _ = salt.decode_b64(&mut iv);

//     return (parsed_hash, iv);
// }
/*
    Deref (any any method that returns a borrowed reference) is for giving access to bytes that already exist in memory,
    exactly in their final form. You can't use it to transform the value on the fly.

    References can only borrow something that has longer-lived storage.
    Variables inside functions are always destroyed before the function returns, so they can't be used after the function returns.
    It's not a matter of syntax, it's just a hard limitation reflecting how the code works
    (if you force it through with unsafe and raw pointers, you're going to get strange bugs and crashes). */

macro_rules! encrypt_folders {
    ($($path:expr),*) => {
        $(
            encrypt_folder($path);
        )*
    };
}
pub fn encrypt_folder(path: &str) {
    for entry in std::fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.is_file() {
            encrypt_file(path.as_path().to_str().unwrap());
        } else if path.is_dir() {
            encrypt_folder(path.as_path().to_str().unwrap());
        }
    }
}

pub fn encrypt_file(path: &str) {
    let mut file = File::open(path).expect("Failed to open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Failed to read file");

    // let encrypted_contents = encrypt(&contents);//TODO

    let mut file = File::create(path).expect("Failed to create file");
    // file.write_all(&encrypted_contents).expect("Failed to write file");
}
