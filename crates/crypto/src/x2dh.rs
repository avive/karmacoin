// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::kdfer::Kdfer;
use crate::utils::{PublicKeyWrapper, StaticSecretWrapper};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::fmt::Debug;
use x25519_dalek::PublicKey;

/// X2DH protocol implementation
/// Double DH algorithm that doesn't include Alice's public ID for privacy considerations.
/// Alice's public identity is known to Bob by obtaining it and a signature in encrypted data
/// sent by Alice to Bob. This is useful to avoid leakage of Alice's ID in clear-text network messages between Alice and Bob as part of the shared secret establishment protocol.

/// Alice's X2DH protocol execution input
pub struct ProtocolInputAlice {
    pub ikb: ed25519_dalek::PublicKey, // Bob's public identity key
    pub pkb: x25519_dalek::PublicKey,  // Bob's public pre-key
    pub b_bundle_id: u64, // the id of bob's identity bundle used to get bob's public keys
}

/// Alice's protocol execution output
#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolOutputAlice {
    pub eka: PublicKey,          // the public key of party a ephemeral key
    pub shared_secret: [u8; 32], // the shared secret created between a nd b
    pub ad: Bytes,               // pub associated_date: [u8],   // AD in the X2DH protocol
}

/// Bob's X2DH protocol execution input
pub struct ProtocolInputBob {
    pub eka: PublicKey,                          // Alice ephemeral x25519 public key
    pub ikb_pair: ed25519_dalek::Keypair,        // Bob's id key pair
    pub pkb_private: x25519_dalek::StaticSecret, // Bob's pre-key private key (pub extractable)
    pub b_bundle_id: u64, // the id of bob's identity bundle used to get bob's public keys
}

/// Bob's protocol output
#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolOutputBob {
    pub shared_secret: [u8; 32], // the shared secret created between a nd b
    pub ad: Bytes,               // pub associated_date: [u8],   // AD in the X2DH protocol
}

/// Compute X2dH AD from eka and ikb
/// eka is Alice's public ephemeral key. ikb is bob's public id
pub fn compute_ad(eka: PublicKey, ikb: ed25519_dalek::PublicKey) -> Bytes {
    let mut hasher = Sha512::new();
    hasher.update(eka.as_bytes().to_vec());
    hasher.update(ikb.as_bytes().to_vec());

    let res = hasher.finalize().to_vec();
    Bytes::from(res.to_vec())
}

/// Execute the X2DH protocol (see X2DH spec).
/// Alice is the protocol initiator. She calls this method to execute the protocol with Bob.
pub fn execute_alice(input: &ProtocolInputAlice) -> ProtocolOutputAlice {
    // ephemeral secret - we use StaticSecret as we need to diffie hellman more than once with it
    let ea_secret = x25519_dalek::StaticSecret::new(&mut rand_core::OsRng);
    // ephemeral public key
    let eka = PublicKey::from(&ea_secret);

    //DH1 = DH(EKA, IKB)
    let ikb_wrapper: PublicKeyWrapper = input.ikb.into();
    let dh1 = ea_secret.diffie_hellman(&ikb_wrapper.0);

    //DH2 = DH(EKA, PKB)
    let dh2 = ea_secret.diffie_hellman(&input.pkb);

    // SK = KDF(DH2 || DH3 )
    let shared_secret = Kdfer::kdf(dh1.as_bytes(), dh2.as_bytes(), None, None).unwrap();

    // AD = Encode(IKA) || Encode(IKB)
    let ad = compute_ad(eka, input.ikb);

    ProtocolOutputAlice {
        eka,
        shared_secret,
        ad,
    }
}

/// Bob's is the receiver of an X2DH protocol request from Alice.
/// He executes the protocol to device the same shared secret output
pub fn execute_bob(input: &ProtocolInputBob) -> ProtocolOutputBob {
    // DH1 = DH(IKB, EKA)
    let ikb_secret: StaticSecretWrapper = (&input.ikb_pair.secret).into();
    let dh1 = ikb_secret.0.diffie_hellman(&input.eka);

    // DH2 = DH(PKB, EKA)
    let dh2 = input.pkb_private.diffie_hellman(&input.eka);

    // SK = KDF(DH2 || DH3 )
    let shared_secret = Kdfer::kdf(dh1.as_bytes(), dh2.as_bytes(), None, None).unwrap();

    // AD = Encode(IKA) || Encode(IKB)
    let ad = compute_ad(input.eka, input.ikb_pair.public);

    ProtocolOutputBob { shared_secret, ad }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base::test_helpers::enable_logger;

    #[test]
    fn test_x2dh_protocol() {
        enable_logger();

        // let alice_id_key_pair = ed25519_dalek::Keypair::generate(&mut rand_core::OsRng);
        let bob_id_key_pair = ed25519_dalek::Keypair::generate(&mut rand_core::OsRng);
        let bob_pre_key_private = x25519_dalek::StaticSecret::new(&mut rand_core::OsRng);
        let bob_pre_key_public: PublicKey = (&bob_pre_key_private).into();

        //let alice_id_pub_key =
        //   ed25519_dalek::PublicKey::from_bytes(alice_id_key_pair.public.as_bytes()).unwrap();

        // Alice protocol execution
        let input_alice = ProtocolInputAlice {
            ikb: bob_id_key_pair.public,
            pkb: bob_pre_key_public,
            b_bundle_id: 0,
        };

        let output_alice = execute_alice(&input_alice);

        // Bob's execution
        let input_bob = ProtocolInputBob {
            eka: output_alice.eka,
            ikb_pair: bob_id_key_pair,
            pkb_private: bob_pre_key_private,
            b_bundle_id: 0,
        };

        let output_bob = execute_bob(&input_bob);

        debug!(
            "Alice's shared secret: {:?}",
            hex::encode(output_alice.shared_secret)
        );
        debug!(
            "Bob's shared secret: {:?}",
            hex::encode(output_bob.shared_secret)
        );

        debug!("AD: {:?}", hex::encode(output_bob.ad.as_ref()));

        assert_eq!(
            output_bob.shared_secret, output_alice.shared_secret,
            "dh failed - different shared secret"
        );

        assert_eq!(
            output_bob.ad.to_vec(),
            output_alice.ad.to_vec(),
            "dh failed - different AD computed"
        );
    }
}
