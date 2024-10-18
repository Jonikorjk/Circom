use nalgebra::Point2;
use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;
use sha2::Digest;

pub mod models;

/// # Elliptic Curve Digital Signature Algorithm
/// The key generation, signing, and verification algorithms used in this project were described in this article: https://ru.wikipedia.org/wiki/ECDSA
///
/// The eliptic curve parameters took from the secp256k1: https://neuromancer.sk/std/secg/secp256k1#
pub struct ECDSA {
    n: BigUint,
    G: Point2<BigUint>,
}

impl ECDSA {
    pub fn new() -> ECDSA {
        ECDSA {
            n: BigUint::parse_bytes(
                b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
                16,
            )
            .expect("should parse bytes n"),
            G: Point2::new(
                BigUint::parse_bytes(
                    b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
                    16,
                )
                .expect("should parse bytes G.x"),
                BigUint::parse_bytes(
                    b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
                    16,
                )
                .expect("should parse bytes G.y"),
            ),
        }
    }
}

impl ECDSA {
    fn generate_value(&self) -> BigUint {
        let mut rng = thread_rng();
        let lbound = BigUint::from(1 as u8);
        let rbound = self.n.clone() - BigUint::from(1 as u8);

        rng.gen_biguint_range(&lbound, &rbound)
    }

    fn hash_message(&self, msg: &str) -> BigUint {
        let mut sha256 = sha2::Sha256::new();
        sha256.update(msg);
        let digest = sha256.finalize();
        BigUint::from_bytes_be(&digest)
    }

    pub fn generate_keypair(&self) -> models::ECDSAKeypair {
        let d = self.generate_value();
        let Q = self.G.clone() * d.clone();

        models::ECDSAKeypair::new(d, Q)
    }

    pub fn sign_message(&self, msg: &str, d: BigUint) -> models::ECDSASignature {
        loop {
            let k = self.generate_value();
            let M = self.G.clone() * k.clone();

            let r = &M.x % &self.n;

            if r == BigUint::ZERO {
                continue;
            }

            let e = self.hash_message(msg);

            let Some(s1) = k.modinv(&self.n) else {
                continue;
            };
            let s2 = e + (&d * r.clone());

            let s = (s1 * s2) % &self.n;

            if s == BigUint::ZERO {
                continue;
            }

            return models::ECDSASignature::new(r, s);
        }
    }

    pub fn verify_signature(
        &self,
        msg: &str,
        Q: Point2<BigUint>,
        signature: &models::ECDSASignature,
    ) -> bool {
        if !(BigUint::from(1 as u8)..self.n.clone()).contains(&signature.r)
            || !(BigUint::from(1 as u8)..self.n.clone()).contains(&signature.s)
        {
            return false;
        }

        let e = self.hash_message(msg);
        let Some(w) = signature.s.modinv(&self.n) else {
            return false;
        };

        let u1 = e * w.clone();
        let u2 = signature.r.clone() * w % &self.n;
        let X = self.G.clone() * u1 + (Q * u2).coords;

        if X.is_empty() {
            return false;
        }

        let v = X.x.clone() % &self.n;

        v == signature.r
    }
}
