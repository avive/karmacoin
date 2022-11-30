# KarmaCoin Server
This repo contains the source code for KarmaCoin server. The server is written in Rust.
The server provides the KarmaCoin API, implements a KarmaCoin blockchain node and a mobile phone numbers verifier.

To learn more about KarmaCoin visit https://karmaco.in

---

## Building
```cargo build```

## Testing
!Use [cargo-nextest](https://nexte.st/) test runner.

```cargo nextest run```

---

## Dev Notes

All timestamps should be in nanosecs using chrono

```rust
use chrono::prelude::*;
let t = Utc::now().timestamp_nanos() as u64
```

#### Architecture

- Uses `xactors` (over `tokio` runtime) actors pattern. Async actors that can return responses to messages.
- Network protocols are defined in `protobufs` language.
- Uses `tonic` and `prost` for grpc api services.
- Xactor used as local lib - enhanced with new features that required low-level integration such as pub/sub.
- High-level system components are all `xactor services` - registered in the system and restarted automatically when called after they crash.
- Store is implemented using `rocksdb`.

- `base` - shared types.
- `crypto` - low-level crypto lib.
- `client` - Simple client with a grpc api for code reuse in testing servers and client-to-client p2p flows.
- `client-app` - Simple terminal client app with support to config file, cli flags and logging.
- `dr` - Implementation of double-ratchet protocol.
- `db` - Adds ttl capabilities to rocksdb data stoe.
- `server` - Server implementation.
- `server-app` - Simple console server app.
- `playground` - Simple interaction scenerios tests.

---

Copyright (c) 2022 by the [KarmaCoin Authors](https://github.com/). This work is licensed under the KarmaCoin License v0.1.0.




