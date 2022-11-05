use crate::karma_coin::karma_coin_core_types::VerifyNumberResponse;
use anyhow::Result;
use ed25519_dalek::Signer;

impl VerifyNumberResponse {

    /// Sign a channel bundle by channel id and by channel creator
    pub fn sign(
        &mut self,
        key_pair: &ed25519_dalek::Keypair,
    ) -> Result<()> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        use crate::karma_coin::karma_coin_core_types::Signature;
        self.signature = Some(Signature {
            scheme_id: 0,
            signature: key_pair.sign(&buf).as_ref().to_vec(),
        });

        let mut buf1 = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf1)?;

        self.signature = Some(Signature {
            scheme_id: 0,
            signature: key_pair.sign(&buf1).as_ref().to_vec(),
        });
        Ok(())
    }
}