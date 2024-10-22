use num_bigint::BigUint;
use rsa::{PaddingScheme, PublicKeyParts};
use sha2::Digest;
use signatures::algorithms::rsa::{rsa::RSA, RSAPrivateKey, RSAPublicKey};

/// Keypair generation, signing message and signature verifying for the RSA implementation (2048 bit)
#[test]
fn own_impl_rsa_basic_test() {
    let rsa = RSA::new();
    let keypair = rsa.generate_keypair().unwrap();

    let message = "plain text";
    let signature = rsa.sign_message(message, keypair.private_key).unwrap();

    let verified = rsa
        .verify_signature(message, keypair.pub_key, signature, false)
        .expect("algorithm should execute");

    assert!(verified)
}

#[test]
fn basic_flow_with_external_keys() {
    let (ext_pub_key, ext_priv_key) = generate_external_rsa_keypair();

    let pub_key = RSAPublicKey::new(
        BigUint::from_bytes_be(&ext_pub_key.e().to_bytes_be()),
        BigUint::from_bytes_be(&ext_pub_key.n().to_bytes_be()),
    );
    let priv_key = RSAPrivateKey::new(
        BigUint::from_bytes_be(&ext_priv_key.n().to_bytes_be()),
        BigUint::from_bytes_be(&ext_priv_key.d().to_bytes_be()),
    );

    let rsa = RSA::new();
    let message = "sha256";

    let signature = rsa.sign_message(&message, priv_key).expect("should sign");
    let verified = rsa
        .verify_signature(message, pub_key, signature, false)
        .expect("algorithm should execute");

    assert!(verified);
}

#[test]
fn verify_signature() {
    // External lib operations
    let (ext_pub_key, priv_key) = generate_external_rsa_keypair();
    let message = "sha256";

    let mut sha256 = sha2::Sha256::new();
    sha256.update(message);
    let digest = sha256.finalize();
    let ext_signature = priv_key
        .sign(
            PaddingScheme::PKCS1v15Sign {
                hash_len: Some(32),
                prefix: Box::new([]),
            },
            &digest,
        )
        .expect("should sign");

    // Own implementation
    let rsa = RSA::new();
    let pub_key = RSAPublicKey::new(
        BigUint::from_bytes_be(&ext_pub_key.e().to_bytes_be()),
        BigUint::from_bytes_be(&ext_pub_key.n().to_bytes_be()),
    );

    let signature = BigUint::from_bytes_be(&ext_signature);

    let verified = rsa
        .verify_signature(message, pub_key, signature, true)
        .expect("algorithm should execute");

    assert!(verified);
}

fn generate_external_rsa_keypair() -> (rsa::RsaPublicKey, rsa::RsaPrivateKey) {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = rsa::RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = rsa::RsaPublicKey::from(&priv_key);

    (pub_key, priv_key)
}
