[[back]](../CSP.md#header-keys)
# Identity
The player identity as a 32 bytes string

Used in the [`connect`](./method.md#connect) method to identify the player<br>
Identity is a server-generated unique id which serve the purpose of *identify*ing the client by the server. It is linked to the player's credentials to allow the retrieval with correct credential of this id with an [`auth`](./method.md#AUTH) method sent by the client.<br>
When retrieved, the client should save it for further usage in the [`connect`](./method.md#connect) request to skip the [`auth`](./method.md#AUTH) requests.

**Key**: `35`

**Sent by**: Client/Server

**Value Type**: string