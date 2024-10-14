mod algorithms;

use algorithms::{ecdsa, rsa::rsa};

fn main() {
    // RSA 2048 bit
    let rsa = rsa::RSA::new();

    let keypair = rsa.generate_keypair().unwrap();

    let message = "penis";
    let signature = rsa.sign_message(message, keypair.private_key).unwrap();

    let verified = rsa
        .verify_signature(message, keypair.pub_key, signature)
        .unwrap();

    println!("RSA verified: {:?}", verified);

    // ECDSA secp256k1
    let ecdsa = ecdsa::ECDSA::new();
    let keypair = ecdsa.generate_keypair();

    let message = "penis";
    let signature = ecdsa.sign_message(message, keypair.d);
    let verified = ecdsa.verify_signature(message, keypair.Q, &signature);

    println!("ECDSA verified: {:?}", verified);
}
