// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cryptography;

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, SaltString},
    Argon2,
};
use cryptography::crypto::{decrypt, encrypt};
use rand::rngs::OsRng;
use std::{
    env,
    fs::File,
    io::{Read, Write},
};

// Learn more about Tauri commands at:
//https://tauri.app/v1/guides/features/command
//https://jonaskruckenberg.github.io/tauri-docs-wip/development/debugging.html

#[tauri::command]
fn encrypt_handler(path: &str, pwd: &str) -> String {
    format!("Rust Backend:Response > > > path: {} , PWD : {}", path, pwd)
    //TODO: here i will have ',' separator for multiple file, folder paths so i need to parse through input and validation (if file or folder)
}

//TOODO: move bellow logic to above cunctions (or combine them into single one )
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![encrypt_handler])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    let pth = env::args()
        .nth(1)
        .expect("1st parameter should be valid file PATH.");

    let out_enc = "encrypted_out.txt";
    let out_dec = "decrypted_out.txt";
    let out_encrypt = format!("<OUT_DIR_PATH>{out_enc}");
    let out_decrypt = format!("<OUT_DIR_PATH>{out_dec}");

    // Open the input and output files
    let mut input_file = File::open(pth).expect("FILE not found at specified location");
    let mut output_file = File::create(out_encrypt).expect("Output location is invalid path");
    let mut output_file2 = File::create(out_decrypt).expect("Output location is invalid path");

    // Read the contents of the input file into a buffer
    let mut input_buffer = Vec::new();
    input_file.read_to_end(&mut input_buffer).unwrap();

    //-------------------------ARAGON
    let password = b"hunter42"; // Bad password; don't actually use!

    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = Argon2::default()
        .hash_password(password, &salt)
        .unwrap()
        .to_string();
    // Verify password against PHC string.
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();

    //Verify TEST
    // assert!(Argon2::default()
    //     .verify_password(password, &parsed_hash)
    //     .is_ok());

    let mut iv: [u8; 16] = [0; 16];
    let _ = salt.decode_b64(&mut iv);

    /* TO CREATE SALT FROM STRING:
         SaltString::from_b64(salt.to_string().as_str());
         let chrs  = salt.to_string(); //as .chars() if i need them
    */

    let encrypted_data = encrypt(&input_buffer, parsed_hash.hash.unwrap().as_bytes(), &iv)
        .ok()
        .unwrap();

    let decrypted_data = decrypt(
        &encrypted_data[..],
        &parsed_hash.hash.unwrap().as_bytes(),
        &iv,
    )
    .ok()
    .unwrap();

    //  let mut output_buffer = Vec::new();
    output_file.write_all(&encrypted_data).unwrap();
    output_file2.write_all(&decrypted_data).unwrap();
}
//check equality in UTests
//  let compare_are_equal = message.as_bytes() == &decrypted_data[..];
//  print!("Are equal {}", compare_are_equal);
//  assert!(compare_are_equal);
