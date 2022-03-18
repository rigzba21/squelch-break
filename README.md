# squelch-break
A Rust library and CLI sending and receiving Constrained Application Protocol (CoAP) 
messages using the Concise Binary Object Representation (CBOR) data format.

Attempting to implement:
* [COAP RFC 7252]( https://datatracker.ietf.org/doc/html/rfc7252#section-3)
* [CBOR RFC 8949](https://cbor.io/)

TODO: 
- [ ] Use [Rust `std::net` primitives](https://doc.rust-lang.org/std/net/#) to reduce dependencies + target binary size 

### Building
`make all`

Now in another terminal, use `nc` to send a UDP message:
```bash
nc -u 127.0.0.1 8080
```
