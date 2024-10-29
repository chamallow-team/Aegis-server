# Server

Contains the start point of the server, the server hub and game handler

read the doc [here](/doc/Server.md)

## Requirements

- last version of OpenSSL (for security reasons)

## Setting up the server

### Environment variables

First, you'll need to create a `.env` file with the following structure:

```dotenv
SECRET_KEY="..."
```

Where:

- `SECRET_KEY`: A random-generated sequence of characters, something that is unique and that no one can guess.
  This
  secret will be used to generate user tokens (the method of authentication).
  For example, something like
  `GxKxZ>uJNhc?'!*^F7-ct/2nw5#<_s:f'h$Evl?sid'UG$rU+PrTUkT"0SD` will be enough secured.

> ⚠️ Remember that your secret key should be private and that NO ONE can ask for it, even the authorities.

### Initiating all files

At the directory where you put your `.env` file, copy the executable file of the server.

Then, simply

```bash
./aegis init
```

It'll create all necessary files and directories and the main database.

### Launching the server

Finally, for launching the server, the only thing required to do is

```bash
./aegis launch --port 11001 --address "0.0.0.0"
```