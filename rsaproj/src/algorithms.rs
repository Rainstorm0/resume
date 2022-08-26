use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};
use num_traits::{One, Zero};
use rand;

pub fn gen_key() -> (BigUint, BigUint, BigUint) {
    let (p, q) = find_2_primes();
    let e = 65537.to_biguint().unwrap();
    let dmod = (p - 1.to_biguint().unwrap()) * (q - 1.to_biguint().unwrap());
    let n = 0.to_biguint().unwrap();
    if (&dmod % &e).is_zero() {
        println!("Not relatively prime");
        let (n, e, d) = gen_key();
        return (n, e, d);
    }
    let (d, _) = eea(&e, &dmod);
    let mut d = d.to_biguint().unwrap(); // careful here...
    d = d % dmod;
    return (n, e, d);
}

fn find_2_primes() -> (BigUint, BigUint) {
    let p = 32.to_biguint().unwrap();
    let q = 15.to_biguint().unwrap();
    return (p, q);
}

// this seems to work
pub fn get_prime(bit_size: u64, trials: i32) -> BigUint {
    let mut rng = rand::thread_rng();
    loop {
        let mut p = rng.gen_biguint(bit_size);
        if ((&p) % 2.to_biguint().unwrap()).is_zero() {
            p = p + 1.to_biguint().unwrap();
        }
        if f_test(&p, trials) {
            return p;
        }
    }
}

// TODO: for some reason, this returns false on primes... not sure why...
// good news is that it is safe and RARELY calls a composite prime.
pub fn f_test(p: &BigUint, trials: i32) -> bool {
    let mut rng = rand::thread_rng();
    for _ in 0..trials {
        let a = rng.gen_biguint(10); // TODO: don't hardcode max
        let psubone = p - 1.to_biguint().unwrap();
        if !a.modpow(&psubone, p).is_one() {
            return false;
        }
    }
    return true;
}

// this can definitely be optimized... but it works for now
pub fn eea(a: &BigUint, b: &BigUint) -> (BigInt, BigInt) {
    let mut q = 0.to_bigint().unwrap();
    let mut r1 = a.to_bigint().unwrap();
    let mut r2 = b.to_bigint().unwrap();
    let mut c1r1 = 1.to_bigint().unwrap();
    let mut c1r2 = 0.to_bigint().unwrap();
    let mut c2r1 = 0.to_bigint().unwrap();
    let mut c2r2 = 1.to_bigint().unwrap();
    while !(&r2.is_zero()) {
        println!("{q} {r1} {c1r1} {c2r1}");
        q = &r1 / &r2;
        let mut temp = (&r1).clone();
        r1 = r2.clone();
        r2 = temp % r2;

        temp = (&c1r2).clone();
        c1r2 = &c1r1 - (&c1r2 * &q);
        c1r1 = temp.clone();

        temp = (&c2r2).clone();
        c2r2 = &c2r1 - (&c2r2 * &q);
        c2r1 = temp.clone();
    }
    println!("{q} {r1} {c1r1} {c2r1}");
    return (c1r1, c2r1);
}

// pub fn encrypt(m: &str, pub_key: &BigUint) -> String {}

// pub fn decrypt(c: &str, priv_key: &BigUint) -> String {}
