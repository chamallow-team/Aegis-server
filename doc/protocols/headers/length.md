[[back]](../CSP.md#header-keys)
# Length
Tells the length of the `data` part.

**Key**: `34`

**Sent by**: Server/Client

**Value Type**: 8 bytes number

**Values**: all numbers


Packets are concidered as corrupted if :
- the `length` header is set and `data` is not provided
- the `length` header is not set and `data` is provided
- the `length` header is bigger than `data`
- the `length` header is smaller than `data`