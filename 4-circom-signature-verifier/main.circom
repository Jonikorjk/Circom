pragma circom  2.0.0;

template Secp256k1VerifySignature() {
    signal input message;
    signal input signature[64];
    signal input publicKey[65];
    signal output {binary} isVerified;

    signal w <== 
}

component main { public[publicKey] } = Secp256k1VerifySignature();
