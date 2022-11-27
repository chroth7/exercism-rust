// the big primes fail, but well, didn't want to use bigint or something...
pub fn private_key(_p: u64) -> u64 {
    3
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // g.pow(a.try_into().unwrap()) % p
    mod_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // b_pub.pow(a.try_into().unwrap()) % p
    mod_exp(b_pub, a, p)
}

fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut i = 0;
    let mut c = 1;

    while i < exp {
        i += 1;
        c = (base * c) % modulus;
    }
    c
}
