## SNP Network Protocol
protocol version: 0.1.0

This folder contains the SNP GRPC services definitions, network protocol, and several higher-level network protocols.

## SNP Protocol Versioning
SNP uses semantic versioning. Each node which uses the UNP Protocol implements a specific version of the protocol and knows what version it implements.

Nodes that send network messages to the nodes MUST specify the protocol version they implement. 

Nodes which receive a SNP network message, should check the caller's protocol version and reject requests that expect responses in protocol format that they don't support. In other words, the data they return is not forward compatible with the caller's implemented protocol version. 

Specifically, nodes will reject a incoming network requests to establish a new DR session with another node iff the protocol version they implement is not forward compatible with the client's implemented version.

All client method calls to GRPC services provided by nodes which do not use the DR session system MUST include the client's protocol semantic version. GRPC service providers MUST reject requests by new protocol versions which they don't provide forward compatibility for and accept requests that they can provide compatible responses for.


Identity Bundles and DialupInfo includes the identity's server or client implemented protocol version.


Copyright (c) 2021 by the [Subnet Authors](https://github.com/subnetter?tab=repositories). This work is licensed under the Subnet License v0.1.0.
