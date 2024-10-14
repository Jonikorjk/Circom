use eyre::ContextCompat;
use num_bigint::BigUint;
use sha2::Digest;

use super::{RSAKeypair, RSAPrivateKey, RSAPublicKey};

/// # RSA Algorithm implementation
/// * Key generation algorithm used from this paper: https://www.simplilearn.com/tutorials/cryptography-tutorial/rsa-algorithm
/// * Sign and verification algorithms used from this paper: [link](https://eitca.org/cybersecurity/eitc-is-acc-advanced-classical-cryptography/digital-signatures/digital-signatures-and-security-services/examination-review-digital-signatures-and-security-services/how-does-the-rsa-digital-signature-algorithm-work-and-what-are-the-mathematical-principles-that-ensure-its-security-and-reliability/)
/// 
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
