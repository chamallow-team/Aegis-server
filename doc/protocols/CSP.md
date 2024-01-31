# Client-Server Protocol (CSP)
- [Header](#header)
  - [control characters](#control-characters)
  - keys
- Data


The CSP follows a header-data format, with reduced informations.

*all bytes represented in the following documentation are decimals*


The separation between header/data is made with [control char](#control-characters):
```
header .. 1 2 .. data
```
it's mandatory to mark the end of an header otherwises it's concidered as a corrupted packet

if no start of data control is found, it means that no data has been sent with the packet

## header
the header use a key-value pair with optimisation in mind:
  - a key is a single value between 32 and 255
  - a value is defined depending of the associated key:
    - **single byte value**: an enum value between 23 and 255
    - **4 bytes number**: a number represented as an ascii hex. eg: `48 97 102 50` == 0af2 (2802)
    - **8 byte number**: same as 4 byte number with more bytes for larger numbers
    - **string**: a way to pass unknown length value, it starts with `string_start` control and ends with `string_end` control.<br>
    everything contained between the controls bytes must be value utf8/ascii values


## Control characters
They are contained between 1 and 31 and define special behavior for the packet, such as header delimitation, value formating...
- `1` : end of header (`header_end`)
- `2` : start of data (`data_start`)
- `3` : start of string (`string_start`)
- `4` : end of string (`string_end`)

## header keys
name|value
----|-----
[METHOD](./headers/method.md) | 32
[SERVER](./headers/server.md) | 33
