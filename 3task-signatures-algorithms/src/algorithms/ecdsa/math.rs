use nalgebra::Point2;
use num_bigint::BigUint;

use super::ECDSA;

// https://learnmeabitcoin.com/technical/cryptography/elliptic-curve/#double
pub trait EcdsaMath {
    fn add_points(&self, q: &Point2<BigUint>, p: &Point2<BigUint>) -> Point2<BigUint>;
    fn double_point(&self, p: &Point2<BigUint>) -> Point2<BigUint>;
    fn mul_point(&self, p1: Point2<BigUint>, scalar: BigUint) -> Point2<BigUint>;
}

impl EcdsaMath for ECDSA {
    fn add_points(&self, p1: &Point2<BigUint>, p2: &Point2<BigUint>) -> Point2<BigUint> {
        if p1 == p2 {
            return self.double_point(p1);
        }

        let slope =
            (&p1.y - &p2.y) * (&p1.x - &p2.x).modinv(&self.p).expect("should inverse") % &self.p;

        let x = (slope.pow(2) - &p1.x - &p2.x) % &self.p;
        let y = ((slope * (&p1.x - &x) - &p1.y)) % &self.p;

        Point2::new(x, y)
    }

    fn double_point(&self, point: &Point2<BigUint>) -> Point2<BigUint> {
        let slope: BigUint = ((3u8 * point.x.pow(2) + &self.a) * (2u8 * &point.y).modinv(&self.p).expect("should inverse")) % &self.p;

        let x = (slope.pow(2) - (2u8 * &point.x)) % &self.p;
        println!("x: {:?} p.x {:?}", x, point.x);
        let y = (slope * (&point.x - &x) - &point.y) % &self.p;

        Point2::new(x, y)
    }

    fn mul_point(&self, p1: Point2<BigUint>, scalar: BigUint) -> Point2<BigUint> {
        let mut current_point = p1.clone();

        let binary_representation = scalar.to_radix_be(2);

        binary_representation.iter().skip(1).for_each(|bit| {
            current_point = self.double_point(&current_point);

            if bit == &1u8 {
                current_point = self.add_points(&current_point, &p1);
            }
        }); 

        current_point
    }
}
