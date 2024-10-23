# ECDSA and RSA signature algorithm implementations.
There is own implementation of the RSA(2048 bit) and the ECDSA(Secp256k1 256 bit)

## ECDSA resources
#### Eliptic curve math operations
There are 2 light articles about how eliptic curve works from the math side. I fully recommend to pass your eyes through them: the [learnmeabitcoin](https://learnmeabitcoin.com/technical/cryptography/elliptic-curve/#double) and [habr](https://habr.com/ru/articles/692072/) articles.

#### Eliptic curve configuration
In this crate we are using parameters of the `Secp256k1` eliptic curve. All the EC parameters were desribed [here](https://neuromancer.sk/std/secg/secp256k1#).

#### Signature algorithms
Key generation, signing and verification signature algorithms were took from [here](https://ru.wikipedia.org/wiki/ECDSA).

## RSA resources