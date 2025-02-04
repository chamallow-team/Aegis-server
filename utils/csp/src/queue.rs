// TODO Packet Queue

pub struct Queue {}

// needs :
//     1. have a "stack" to queue incomming packet if the client handles packet slower that it parse them
//     2. have a queue to keep track of packet that need to get a response
//
// how ?
// 1. :
//     could be easilly implemented with a double ended queue, which behave like a stack.
//     it'll be efficient for out need (will cache up to 10 packets, if more it's a server outage ?)
//     packet are handled the same order we receives them with the concept of a stack (bottom is consumming while top is adding)
//
// 2. :
//     a map could do the trick with key => Id, value => Packet where the Id is the header of the packet
//     don't know if hashing a number (u64) can be efficient, and would not lead to big memory usage
