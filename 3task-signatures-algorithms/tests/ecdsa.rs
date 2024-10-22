use signatures::algorithms::ecdsa::ECDSA;

#[test]
fn keypair_gen() {
    let ecdsa = ECDSA::new();

    let keypair = ecdsa.generate_keypair();
    let left_side = keypair.q_point.y.pow(2) % &ecdsa.p;
    let right_side = (keypair.q_point.x.pow(3) + 7i8) % &ecdsa.p;

    println!("left_side: {}, right_side: {}", left_side, right_side);

    // Public key is a dot in secp256k1
    assert!(left_side.eq(&right_side));

    // private key is in range 0..n
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

