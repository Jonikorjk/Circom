use eyre::bail;
use nalgebra::Point2;
use num_bigint::BigUint;
use secp256k1::{ffi::PublicKey, hashes::hex::DisplayHex, Secp256k1};
use signatures::algorithms::ecdsa;

/// Keypair generation, signing message and verifying signature for ecdsa
#[test]
fn ecdsa_basic_test() {
    let ecdsa = ecdsa::ECDSA::new();
    let keypair = ecdsa.generate_keypair();

    let message = "plain text";
    let signature = ecdsa.sign_message(message, keypair.d);
    let verified = ecdsa.verify_signature(message, keypair.Q, &signature);

    assert!(verified);
}

#[test]
fn private_key_generation() {
    let ecdsa_impl = ecdsa::ECDSA::new();
    let keypair = ecdsa_impl.generate_keypair();

    let binding = keypair.d.to_bytes_be();
    let bytes: &[u8] = binding.as_slice();

    let _ = secp256k1::SecretKey::from_byte_array(bytes.try_into().expect("should convert"))
        .expect("should create secret key");
}

#[test]
fn sign_and_verify() {
    let ecdsa_impl = ecdsa::ECDSA::new();
    let pk = ecdsa_impl.generate_keypair();

    let keypair = ecdsa_impl.generate_keypair();

    let message = "plain text";
    let signature = ecdsa_impl.sign_message(message, keypair.d);

    let secp = Secp256k1::new();

    let pub_key = create_public_key_from_point(&keypair.Q).unwrap();
}

fn create_public_key_from_point(q: &nalgebra::Point2<BigUint>) -> eyre::Result<PublicKey> {
    // Check if the input coordinates are 32 bytes long

    println!("pizda {}", q.x.to_bytes_be().len());

    if q.x.to_bytes_be().len() != 32 || q.y.to_bytes_be().len() != 32 {
        bail!("invalid length of point coordinates");
    }

    // Create a point representation
    let mut pubkey_bytes = Vec::with_capacity(33);

    // Determine if y is even or odd for compression
    let y_is_even = q.y.to_bytes_be()[31] & 1 == 0;
    let prefix = if y_is_even { 0x02 } else { 0x03 };

    // Prepend the prefix to the x coordinate
    pubkey_bytes.push(prefix);
    pubkey_bytes.extend_from_slice(q.x.to_bytes_be().as_slice());

    // Create and return the PublicKey
    Ok(unsafe { PublicKey::from_array_unchecked(pubkey_bytes.as_slice().try_into().unwrap()) })
}
