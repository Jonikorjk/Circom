use math::EcdsaMath;
use nalgebra::Point2;
use num_bigint::{BigInt, BigUint, RandBigInt};
use rand::thread_rng;
use sha2::Digest;

pub mod math;
pub mod models;

/// # Elliptic Curve Digital Signature Algorithm
/// The key generation, signing, and verification algorithms used in this project were described in this article: https://ru.wikipedia.org/wiki/ECDSA
///
/// The eliptic curve parameters took from the secp256k1: https://neuromancer.sk/std/secg/secp256k1#
pub struct ECDSA {
    pub n: BigInt,
    pub g_point: Point2<BigInt>,
    pub p: BigInt,
    a: BigInt,
}

impl ECDSA {
    pub fn new() -> ECDSA {
        ECDSA {
            n: BigInt::parse_bytes(
                b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
                16,
            )
            .expect("should parse bytes n"),
            g_point: Point2::new(
                BigInt::parse_bytes(
                    b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
                    16,
                )
                .expect("should parse bytes G.x"),
                BigInt::parse_bytes(
                    b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
                    16,
                )
                .expect("should parse bytes G.y"),
            ),
            p: BigInt::parse_bytes(
                b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
                16,
            )
            .expect("should parse p"),
            a: BigInt::parse_bytes(
                b"0000000000000000000000000000000000000000000000000000000000000000",
                16,
            )
            .expect("should parse a"),
        }
    }
}

impl ECDSA {
    fn generate_value(&self) -> BigInt {
        let mut rng = thread_rng();

        rng.gen_bigint_range(&1i8.into(), &self.n)
    }

    fn hash_message(&self, msg: &str) -> BigInt {
        let mut sha256 = sha2::Sha256::new();
        sha256.update(msg);
        let digest = sha256.finalize();
        BigInt::from_bytes_be(num_bigint::Sign::NoSign, &digest)
    }

    pub fn generate_keypair(&self) -> models::ECDSAKeypair {
        let d = self.generate_value();
        let q_point = self.mul_point(self.g_point.clone(), d.clone());

        models::ECDSAKeypair::new(d, q_point)
    }

    pub fn sign_message(&self, msg: &str, d: BigInt) -> models::ECDSASignature {
        loop {
            let k = self.generate_value();
            let kg_point = self.mul_point(self.g_point.clone(), k.clone());

            let r = &kg_point.x % &self.n;

            if r == BigInt::ZERO {
                continue;
            }

            let e = self.hash_message(msg);

            let Some(s1) = k.modinv(&self.n) else {
                continue;
            };

            let s2 = (&d * &r + e) % &self.n;

            let s = (s1 * s2) % &self.n;

            if s == BigInt::ZERO {
                continue;
            }

            return models::ECDSASignature::new(r, s);
        }
    }

    pub fn verify_signature(
        &self,
        msg: &str,
        q_point: Point2<BigInt>,
        signature: &models::ECDSASignature,
    ) -> bool {
        let range = 1u8.into()..self.n.clone();

        if !range.contains(&signature.r) || !range.contains(&signature.s) {
            return false;
        }

        let e = self.hash_message(msg);
        let Some(w) = signature.s.modinv(&self.n) else {
            return false;
        };

        let u1 = e * &w;
        let u2 = &signature.r * w % &self.n;

        let u1_g = self.mul_point(self.g_point.clone(), u1);
        let u2_q = self.mul_point(q_point.clone(), u2);

        let x_point = self.add_points(&u1_g, &u2_q);
        if x_point.is_empty() {
            return false;
        }

        let v = &x_point.x % &self.n;

        println!("v: {:?}", v);
        println!("signature.r: {:?}", signature.r);

        v == signature.r
    }
}
