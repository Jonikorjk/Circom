use eyre::Ok;
use num_bigint::BigUint;
pub mod rsa;

#[derive(Debug)]
pub struct RSAKeypair {
    pub private_key: RSAPrivateKey,
    pub pub_key: RSAPublicKey,
}

impl RSAKeypair {
    pub fn new(n: BigUint, e: BigUint, d: BigUint) -> RSAKeypair {
        RSAKeypair {
            private_key: RSAPrivateKey::new(n.clone(), d),
            pub_key: RSAPublicKey::new(e, n),
        }
    }

    pub fn from_keys(pub_key: RSAPublicKey, private_key: RSAPrivateKey) -> RSAKeypair {
        RSAKeypair {
            private_key,
            pub_key,
        }
    }
}

#[derive(Debug)]
pub struct RSAPrivateKey {
    pub n: BigUint,
    pub d: BigUint,
}

impl RSAPrivateKey {
    pub fn new(n: BigUint, d: BigUint) -> RSAPrivateKey {
        RSAPrivateKey { n, d }
    }
}

#[derive(Debug)]
pub struct RSAPublicKey {
    pub e: BigUint,
    n: BigUint,
}

impl RSAPublicKey {
    pub fn new(e: BigUint, n: BigUint) -> RSAPublicKey {
        RSAPublicKey { e, n }
    }

    pub fn from_der(string: &str) -> eyre::Result<RSAPublicKey> {
        let bytes = openssl::base64::decode_block(string)?;
        let pub_key = openssl::rsa::Rsa::public_key_from_der(&bytes)?;

        let e = BigUint::from_bytes_be(&pub_key.e().to_vec());
        let n = BigUint::from_bytes_be(&pub_key.n().to_vec());

        Ok(RSAPublicKey::new(e, n))
    }
}
