/**

  This script will be called to create the database.

  Please note that this script will erase every data

 */
DROP TABLE IF EXISTS users;

CREATE TABLE users
(
    -- For example, 'Lynn', 'Little'
    nickname   VARCHAR(64) NOT NULL,
    -- For example, 'lynn', 'little_endian', it'll be used for logging in
    username   VARCHAR(64) NOT NULL,

    passwd     CHAR(64),
    hashed_jwt CHAR(64), -- We'll store the hashed jwt so that no one can access to the jwt


    PRIMARY KEY (username)
);

--
-- FOR TEST PURPOSES

INSERT INTO users (nickname, username, passwd, hashed_jwt)
VALUES ('Lynn', 'lynn', '598f7a741a1e3a05654d346033571fda567af6dc2bf099b34b930171519d995f',
        'd2b85524c259e57313f6b4dbcec3a7067eee2bbd4cb6f9b87ae060e7bd9d5b72');