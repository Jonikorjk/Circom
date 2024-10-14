use eyre::ContextCompat;
use num_bigint::BigUint;
use sha2::Digest;

use super::{RSAKeypair, RSAPrivateKey, RSAPublicKey};

/// 2048 bits impl
pub struct RSA {
    exp: BigUint,
}

impl RSA {
    pub fn new() -> RSA {
        RSA {
            exp: BigUint::from(65537 as u32),
        }
    }

    pub fn generate_keypair(&self) -> eyre::Result<RSAKeypair> {
        let Ok(p) = glass_pumpkin::prime::new(1024) else {
            eyre::bail!("Failed to generate prime p");
        };

        let Ok(q) = glass_pumpkin::prime::new(1024) else {
            eyre::bail!("Failed to generate prime q");
        };

        let n = p.clone() * q.clone();
        let z = (p - BigUint::from(1 as u8)) * (q - BigUint::from(1 as u8));

        let d = self.exp.modinv(&z).wrap_err("failed to calculate d")?;

        Ok(RSAKeypair::new(n, self.exp.clone(), d))
    }

    fn hash_message(&self, msg: &str) -> eyre::Result<BigUint> {
        let mut sha256 = sha2::Sha256::new();
        sha256.update(msg);
        let digest = sha256.finalize();
        let message = BigUint::from_bytes_be(&digest);

        Ok(message)
    }

    pub fn sign_message(&self, msg: &str, pk: RSAPrivateKey) -> eyre::Result<BigUint> {
        let hash_message = self.hash_message(msg)?;
        let signature = hash_message.modpow(&pk.d, &pk.n);

        Ok(signature)
    }

    pub fn verify_signature(
        &self,
        msg: &str,
        pub_key: RSAPublicKey,
        signature: BigUint,
    ) -> eyre::Result<bool> {
        let message = self.hash_message(msg)?;
        let preimage_msg = signature.modpow(&pub_key.e, &pub_key.n);

        Ok(message == preimage_msg)
    }
}
