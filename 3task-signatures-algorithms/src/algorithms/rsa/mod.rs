use eyre::Ok;
use num_bigint::BigUint;
use openssl::bn::BigNum;
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

    pub fn from_pem(string: &str) -> eyre::Result<RSAPrivateKey> {
        let private_key = openssl::rsa::Rsa::private_key_from_pem(string.as_bytes())?;

        let d = BigUint::from_bytes_be(&private_key.d().to_vec());
        let n = BigUint::from_bytes_be(&private_key.n().to_vec());

        Ok(RSAPrivateKey::new(n, d))
    }

    pub fn to_pem(&self) -> eyre::Result<String> {
        let n: BigNum = BigNum::from_slice(&self.n.to_bytes_be())?;
        let d = BigNum::from_slice(&self.d.to_bytes_be())?;
        let e = BigNum::from_dec_str("65537")?;

        let pk = openssl::rsa::RsaPrivateKeyBuilder::new(n, e, d)?.build();
        let pem = pk.private_key_to_pem()?;

        // Ok()
        Ok("".to_string())
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
