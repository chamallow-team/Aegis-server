/**

  This script will be called to create the database.

  Please note that this script will erase every data

 */
DROP TABLE IF EXISTS users;

CREATE TABLE users
(
    -- For example, 'Lynn', 'Little'
    nickname VARCHAR(64) NOT NULL,
    -- For example, 'lynn', 'little_endian', it'll be used for logging in
    username VARCHAR(64) NOT NULL,

    passwd   CHAR(64),


    PRIMARY KEY (username)
);