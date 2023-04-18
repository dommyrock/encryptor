CREATE TABLE IF NOT EXISTS encryption_keys
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    file_path   VARCHAR(500)        NOT NULL,
    key         VARCHAR(50)         NOT NULL,
    salt        VARCHAR(50)         NOT NULL,
    active      BOOLEAN             NOT NULL DEFAULT 0
);

-- insert test key
INSERT INTO encryption_keys
VALUES(1, 'C:/', 'ADS123', '123123123', 1)