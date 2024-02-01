[[back]](../CSP.md#header-keys)
# Methods

Define the action performed by the client/server, same use as HTTP method.

**Key**: `32`

**Sent by**: Server/Client

**Value Type**: single byte

**Values**:
name                      |value | sent by        | description 
--------------------------|------|----------------|--------------
[CONNECT](#connect)       |  32  | Client         | ask the server for connection to a game
[AUTH](#auth)             |  33  | Client/Server  | `Server`: send client identity of the client<br>`Client`: send login informations
[DISCONNECT](#disconnect) |  34  | Client/Server  | send a disconnection notice
[ADMIN](#admin)           |  35  | Client/Server  | `Client`: send an admin action<br>`Server`: respond to an admin action
[UPDATE](#update)         |  36  | Server         | send a game update
[ACTION](#action)         |  37  | Client/Server  | `Client`: send game action<br>`Server`: respond to a game action
[ERROR](#error)           |  38  | Client/Server  | send an error message
[STATE](#state)           |  39  | Server         | send all informations about the game to allow initial loading of the client

FIXME should the headers allways contains the identity ?

# CONNECT
**headers**:
- [Server](./server.md) : identify the server you want to connect to
- [Identity](./identity.md) : if the client has the login information of the player
- [Update](./update.md) : if the client is making his first connection to the server

**data**: None

# AUTH
**header**: 
  - [Length](./length.md) : the data exact bytes length

**data**:
  - username : player's unique username on the hub
  - passwd : client's generated password (avoid the use of existing user's passords)
  - create : boolean

# DISCONNECT
**header**:
  - [Length](./length.md) : the data exact bytes length

**data**:
  - reason : can be empty, but usually tells the reason of the disconnection (usefull is disconnection is send by the server)

# ADMIN
**header**:
  - [Id](./id.md) : identify an admin action
  - [Length](./length.md) : the data exact bytes length

**data**:
  - TODO : define a admin action packet

# UPDATE
**header**:
  - [Length](./length.md) : the data exact bytes length

**data**:
  - TODO : define an update packet

# ACTION
**header**:
  - [Id](./id.md) : identify an action
  - [Length](./length.md) : the data exact bytes length

**data**:
  - TODO : define an action packet

# ERROR
**header**:
  - [Id](./id.md) : link to a client' action if this was performed by an action/admin
  - [Length](./length.md) : the data exact bytes length

**data**:
  - id : an error constant to be used by the program to adjust behavior
  - message : a printable message for the user
  
# STATE
**header**:
  - [Update](./update.md) : does this packet contains requested (or not) config/assets/mods
  - [Length](./length.md) : the data exact bytes length

**data**:
  - TODO : define an update packet