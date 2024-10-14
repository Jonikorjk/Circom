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
}

#[derive(Debug)]
pub struct RSAPrivateKey {
    n: BigUint,
    d: BigUint,
}

impl RSAPrivateKey {
    pub fn new(n: BigUint, d: BigUint) -> RSAPrivateKey {
        RSAPrivateKey { n, d }
    }
}

#[derive(Debug)]
pub struct RSAPublicKey {
    e: BigUint,
    n: BigUint,
}

impl RSAPublicKey {
    pub fn new(e: BigUint, n: BigUint) -> RSAPublicKey {
        RSAPublicKey { e, n }
    }
}
