use nalgebra::Point2;
use num_bigint::BigUint;

pub struct ECDSAKeypair {
    pub d: BigUint,
    pub q_point: Point2<BigUint>,
}

impl ECDSAKeypair {
    pub fn new(d: BigUint, q_point: Point2<BigUint>) -> ECDSAKeypair {
        ECDSAKeypair { d, q_point }
    }
}

pub struct ECDSASignature {
    pub r: BigUint,
    pub s: BigUint,
}

impl ECDSASignature {
    pub fn new(r: BigUint, s: BigUint) -> ECDSASignature {
        ECDSASignature { r, s }
    }
}
