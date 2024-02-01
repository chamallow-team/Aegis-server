# Server-Game Protocol (SGP)
- [Usage](#usage)
- [Header](#header)
- [Data](#data)
- [Example](#example)
- [Flow](#flow)

## Usage
This protocol is used for communication between the Server and the Game ([see definitions](../Server.md#definitions)).

It follows the same semantics of the [CSP](./CSP.md), but with different communication flow. So you *should* start by getting familliar with it before reading this file.

The data part of the packet is always the same in the CSP and the SGP, only the header change.

## Header
The server handle conversion from CSP to CGP and conversely.

Main differences with CSP:
  - the Game nevers get to see an [`auth`](./headers/method.md#auth) request, always handled by the server
  - all CGP header have a [`identify`](./headers/identity.md) field

## Data
 TODO : define data

## Example
 TODO : define CGP examples

## Flow
This is an example of a basic connection between a Server and a Game, displaying the methods used

![csp methods flow](../assets/sgp.drawio.svg)


You can read a basic example [here](./example/)