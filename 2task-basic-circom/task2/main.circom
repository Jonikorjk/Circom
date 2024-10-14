pragma circom 2.0.0;

include "node_modules/circomlib/circuits/poseidon.circom";


///`a b c` are rerepresented like a: `preimage[0] = a, preimage[1] = b etc.`
template Main(size) {
    signal input preimage[size];
    signal input hash; 
    
    component poseidon = Poseidon(size);
    poseidon.inputs <== preimage;
    hash === poseidon.out;
}

component main { public[hash] } = Main(3);