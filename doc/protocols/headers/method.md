[[back]](../CSP.md#header-keys)
# Methods

Define the action performed by the client/server, same use as HTTP method.

**Key**: `32`

**Sent by**: Server/Client

**Value Type**: single byte

**Values**:
name        |value | sent by        | description 
------------|------|----------------|--------------
CONNECT     |  32  | Client         | ask the server for connection to a game
AUTH        |  33  | Client/Server  | `Server`: send client identity of the client<br>`Client`: send login informations
DISCONNECT  |  34  | Client/Server  | send a disconnection notice
ADMIN       |  35  | Client/Server  | `Client`: send an admin action<br>`Server`: respond to an admin action
UPDATE      |  36  | Server         | send a game update
ACTION      |  37  | Client/Server  | `Client`: send game action<br>`Server`: respond to a game action
ERROR       |  38  | Client/Server  | send an error message
STATE       |  39  | Server         | send all informations about the game to allow initial loading of the client