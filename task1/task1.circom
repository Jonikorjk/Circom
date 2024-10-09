pragma circom 2.0.0;

template Pow(nTimes) { 
    signal input value;
    signal output valuePoweringCycles[nTimes];

    valuePoweringCycles[0] <== value ** 2;
    for (var i = 1; i < nTimes; i++) {
        valuePoweringCycles[i] <== valuePoweringCycles[i - 1] * value;
    }
}

// a6+7b(a2+b)+42 = N
template Main() {
    signal input a;
    signal input b;
    signal input N; 

    signal powCyclesOfA[6];
    signal bracketsResponse; // 7b(a2 + b)
    signal response;

    // Initializing powering of `a` to 6 
    component pow = Pow(6);
    pow.value <== a;
    powCyclesOfA <== pow.valuePoweringCycles;

    bracketsResponse <== 7 * b * (powCyclesOfA[1] + b);
    response <== powCyclesOfA[5] + bracketsResponse + 42;
    N === response;
}

component main { public[N] } = Main();
