use nalgebra::Point2;
use num_bigint::BigInt;
use signatures::algorithms::ecdsa::models::{ECDSAKeypair, ECDSASignature};
use signatures::algorithms::ecdsa::ECDSA;
use secp256k1::{Message, Secp256k1};
use secp256k1::hashes::{sha256, Hash};

#[test]
fn keypair_gen() {
    let ecdsa = ECDSA::new();

    let keypair = ecdsa.generate_keypair();
    let left_side = keypair.q_point.y.pow(2) % &ecdsa.p;
    let right_side = (keypair.q_point.x.pow(3) + 7i8) % &ecdsa.p;

    println!("left_side: {}, right_side: {}", left_side, right_side);

    // Public key is a dot in secp256k1
    assert!(left_side.eq(&right_side));

    // private key is in range [1..n)
    assert!((0i8.into()..ecdsa.n).contains(&keypair.d));
}

#[test]
fn test_sign_and_verify() {
    let ecdsa = ECDSA::new();
    let keypair = ecdsa.generate_keypair();
    let msg = "Hello, world!";
    let signature = ecdsa.sign_message(msg, keypair.d);
    assert!(ecdsa.verify_signature(msg, keypair.q_point, &signature));
}

#[test]
fn test_verify_signature()  {
    let ext_secp = Secp256k1::new();
    let (secret_key, public_key) = ext_secp.generate_keypair(&mut rand::thread_rng());

    let digest = sha256::Hash::hash("Hello World!".as_bytes());
    let message = Message::from_digest(digest.to_byte_array());
    let signature = ext_secp.sign_ecdsa(&message, &secret_key);
    let serialized_ext_signture = signature.serialize_compact();

    let own_signature = ECDSASignature { 
        r: BigInt::from_bytes_be(num_bigint::Sign::Plus,  &serialized_ext_signture[0..32]),
        s: BigInt::from_bytes_be(num_bigint::Sign::Plus,  &serialized_ext_signture[32..64]),
    };
    let uncompressed_pub_key = public_key.serialize_uncompressed();

    let own_keypair = ECDSAKeypair { 
        d: BigInt::from_bytes_be(num_bigint::Sign::Plus,  &secret_key.secret_bytes()),
        q_point: Point2::new(
            BigInt::from_bytes_be(num_bigint::Sign::Plus,  &uncompressed_pub_key[1..33]),
            BigInt::from_bytes_be(num_bigint::Sign::Plus,  &uncompressed_pub_key[33..65]),
        )
    };

    let secp = ECDSA::new();
    let verified = secp.verify_signature("Hello World!", own_keypair.q_point, &own_signature);

    assert!(verified);
}