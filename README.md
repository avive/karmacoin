# Introducing KarmaCoin
A cryptocurrency designed for giving first and payments later. Provides a first-rate mobile-native user experience for the rest of us. Becomes valuable out of real-world value created by usage and not by speculation and hype.

# KarmaCoin - the coin we all need
The world needs a global decentralized cryptocurrency that is actually used by millions of everyday people to fulfill the full potential of crypto as an alternative to national currencies.

# The social graph we need
The world needs a global, objective and decentralized social graph of core relationships between people and between people and orgs in order to create meaningful digital decentralized identities. The identities are critical for creating an impactful web3 future.

# A solid open source blockchain
KarmaCoin is maintained by a permissioned and decentralized modern blockchain technology.

# A coin for all of us
People use one simple, mobile native wallet app and don't have to deal with any crypto keys.


# Hello Karma Score
Karma score is a measure of a person's positive character traits. It is created by real people appreciating other people using the KarmaCoin mobile app.

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




