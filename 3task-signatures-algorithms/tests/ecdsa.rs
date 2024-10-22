use num_bigint::BigUint;
use signatures::algorithms::ecdsa::{self, ECDSA};

#[test]
fn keypair_gen() {
    let ecdsa = ECDSA::new();

    let keypair = ecdsa.generate_keypair();
    let left_side = keypair.q_point.y.pow(2) % &ecdsa.p;
    let right_side: BigUint = (keypair.q_point.x.pow(3) + 7u8) % &ecdsa.p;

    println!("left_side: {}, right_side: {}", left_side, right_side);

    assert!(left_side.eq(&right_side));
}

#[test]
fn test_group_generator() {
    let ecdsa = ECDSA::new();

    let left_side = ecdsa.g_point.y.pow(2) % &ecdsa.p;
    let right_side: BigUint = (ecdsa.g_point.x.pow(3) + 7u8) % &ecdsa.p;

    println!("left_side: {}, right_side: {}", left_side, right_side);

    assert!(left_side.eq(&right_side));
}
