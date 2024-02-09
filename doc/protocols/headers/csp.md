[[back]](../CSP.md#header-keys)
# Csp
Sent in the [`connect`](./method.md#connect) to specify the csp version used by the client

**Key**: `40`

**Sent by**: Client

**Value Type**: single byte

 Version   |Value | Compatible with
-----------|------|-----------------
 1.0       | `32` | *

Versioning follows a strict pattern of `n.n` [e.g. 9.6, 3.0...] or `nn` [e.g. 96, 30...]. In a packet, it's represented as a strict single byte value, but internally, the software is free to use dotted versioning or not. 