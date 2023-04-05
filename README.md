# KarmaCoin Server
This repo contains the source code for KarmaCoin server. The server is written in Rust.
The server provides the KarmaCoin API, implements a KarmaCoin blockchain node and a mobile phone numbers verifier.

To learn more about KarmaCoin visit https://karmaco.in

---

## Setup
- Clone this repo
- Install rust via `rustup` - stable toolchain - default installation options
- Install `cargo-nextest`. See https://nexte.st/book/pre-built-binaries 


### Ubuntu
Install the following packages via apt-get or similar:
- build-essential
- pkg-config
- libssl-dev
- protobuf-compiler
- libclang-dev
 

## Building

### Building a dev build
```cargo build```

### Building for release
```cargo build --release```

## Testing
- Make sure you have a valid `verifier_config.yaml` file in `crates/server/` with the following syntax:

```YAML
verifier.sms_gateway:
  auth_value: Basic [TWILLO_AUTH_TOKEN]
  from_number: [TWILLO_SEND_SMS_FROM_NUMBER]
  api_endpoint: [TWILLO_API_ENDPOINT]
```

Check `crates/server/verifier_config_template.yaml` for an example.

- Use [cargo-nextest](https://nexte.st/) runner.


```cargo nextest run --test-threads 1```

## Running

To start a server that runs the KarmaCoin blockchain node and provides the KarmaCoin API and verifier API, create a config file `verifier.yaml` with the authentication tokens for the service providers used by the verifier, and provide the path to the config file to the server app.

### Running a dev build
```bash
cargo build
./target/debug/server-app -c verifier.yaml
```

### Running a release build

```bash
cargo build -- release
./target/debug/server-app -c verifier.yaml
```



---

## Dev Notes

### Protos for downstream Dart repos
This repo contains the canonical protobufs definitions for the Karma Coin APIs. To generate protos for other Karma Coin projects in Dart, first enable the dart protoc plugin:

```bash
dart pub global activate protoc_plugin
```
Next, run the following commands from `[project_root]/crates/base/proto`.

```bash
mkdir dart
protoc --dart_out=grpc:dart  karma_coin/core_types/*.proto
```

and copy over the generated files to your Dart project.

### Timestamps
All timestamps should be in miliseconds using chrono. Use milliseonds in clients when working with timestamps.

```rust
use chrono::prelude::*;
let t = Utc::now().timestamp_millis() as u64;
```

### Xactor Usage 
Xactor (unlike Actix) gives us nice and clean async syntax for actor messages. However, be aware that calling an actor message from the body of the same actor's message handler, will deadlock. It is easy to spend hours on such bugs. Just factor out impl into an async fn and call it from different message handlers....

### Architecture
- Uses `xactors` (over `tokio` runtime) actors pattern. Async actors that can return responses to messages.
- Network protocols are defined in `protobufs` language.
- Uses `tonic` and `prost` for grpc api services.
- Xactor used as local lib - enhanced with new features that required low-level integration such as pub/sub.
- High-level system components are all `xactor services` - registered in the system and restarted automatically when called after they crash.
- Store is implemented using `rocksdb`.

### Code Structure
- `base` - shared types.
- `crypto` - low-level crypto lib.
- `client` - Simple client with a grpc api for code reuse in testing servers and client-to-client p2p flows.
- `client-app` - Simple terminal client app with support to config file, cli flags and logging.
- `db` - Adds ttl capabilities to rocksdb data stoe.
- `server` - Server implementation.
- `server-app` - Simple console server app.

---

Copyright (c) 2022 by the KarmaCoin Authors. This work is licensed under the [KarmaCoin License](https://github.com/karma-coin/.github/blob/main/LICENSE).




