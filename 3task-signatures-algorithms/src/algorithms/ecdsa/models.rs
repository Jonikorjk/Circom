use nalgebra::Point2;
use num_bigint::BigInt;

pub struct ECDSAKeypair {
    pub d: BigInt,
    pub q_point: Point2<BigInt>,
}

impl ECDSAKeypair {
    pub fn new(d: BigInt, q_point: Point2<BigInt>) -> ECDSAKeypair {
        ECDSAKeypair { d, q_point }
    }
}

pub struct ECDSASignature {
    pub r: BigInt,
    pub s: BigInt,
}

impl ECDSASignature {
    pub fn new(r: BigInt, s: BigInt) -> ECDSASignature {
        ECDSASignature { r, s }
    }
}
