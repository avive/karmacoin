// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use bytes::{BufMut, BytesMut};
use ed25519_dalek::{Keypair, KEYPAIR_LENGTH};

use crate::karma_coin::karma_coin_core_types::{KeyPair, PrivateKey, PublicKey};

impl KeyPair {
    pub fn to_ed2559_kaypair(&self) -> Keypair {
        let mut buf = BytesMut::with_capacity(KEYPAIR_LENGTH);

        buf.put(self.private_key.as_ref().unwrap().key.as_slice());
        buf.put(self.public_key.as_ref().unwrap().key.as_slice());

        Keypair::from_bytes(buf.as_ref()).unwrap()
    }

    pub fn new() -> Self {
        let pair = Keypair::generate(&mut rand_core::OsRng);
        KeyPair {
            private_key: Some(PrivateKey {
                key: pair.secret.as_bytes().to_vec(),
            }),
            public_key: Some(PublicKey {
                key: pair.public.as_bytes().to_vec(),
            }),
            scheme: 0,
        }
    }
}
