# squelch-break
A Rust CLI for sending and receiving Constrained Application Protocol (CoAP) 
messages using the Concise Binary Object Representation (CBOR) data format.


### Building
`make all`

Now in another terminal, use `nc` to send a UDP message:
```bash
nc -u 127.0.0.1 8080
```