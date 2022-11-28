# KarmaCoin Server and Verifier
This repo includes the source code for KarmaCoin api server, blockchain node and mobile phone numbers verifier.
To learn more about KarmaCoin visit https://karmaco.in

---

## Building
```cargo build```

## Testing
```cargo test```

---

## Dev Notes

All timestamps should be in nanosecs using chrono

```rust
use chrono::prelude::*;
let t = Utc::now().timestamp_nanos() as u64
```

#### Architecture

- Uses `xactors` (over `tokio` runtime) actors pattern. Async actors that can return responses to messages.
- Uses `tonic` and `prost` for grpc
- Xactor used as local lib - enhanced with new features that required low-level integration such as pub/sub.
- High-level system components are all `xactor services` - registered in the system and restarted automatically when called after they crash.
- Store is implemented using `rocksdb`.
- Network protocols are defined in `protobufs` language.

- `base` - shared types.
- `crypto` - low-level crypto lib
- `common` - shared functionality between a client and server that uses types and base, crypto and other libs.
- `client` - Simple client with a grpc api for code reuse in testing servers and client-to-client p2p flows.
- `client-app` - Simple terminal client app with support to config file, cli flags and logging.
- `dr` - Implementation of double-ratchet protocol.
- `server` - Server implementation.
- `server-app` - Simple console server app.

---

Copyright (c) 2021 by the [KarmaCoin Authors](https://github.com/). This work is licensed under the KarmaCoin License v0.1.0.




