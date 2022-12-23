# squelch-break
A small network framework using authenticated + encrypted messages.

## Overview

### Roadmap
Attempting to implement for messages:
* [COAP RFC 7252]( https://datatracker.ietf.org/doc/html/rfc7252#section-3)
* [CBOR RFC 8949](https://cbor.io/)

### Development

#### Key Exchange Request Message
```bash
# start a netcat UDP session
nc -u 127.0.0.1 3156

# send a key exchange request message
{"message_type": 1, "uuid": "0F0E99A1-7D65-47EB-BFBF-9B379B4CAA58", "message_payload": ""}
```
