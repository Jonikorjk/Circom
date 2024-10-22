use log::logger;
use signatures::algorithms::ecdsa::ECDSA;

fn main() {
    let ecdsa = ECDSA::new();
    let keypair = ecdsa.generate_keypair();
}
