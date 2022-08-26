use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand;

mod algorithms;

fn main() {
    let mut rng = rand::thread_rng();

    // let mut num: BigUint = 5.to_biguint().unwrap();
    // println!("Big int is: {num}");
    // num = num + BigUint::from(3 as u8);
    // println!("Big int is now: {num}");

    // let a = rng.gen_biguint(10);
    // println!("Rand bigint is {a}");

    // let power = num.modpow(&BigUint::from(3 as u8), &BigUint::from(100 as u8));
    // println!("8^3 % 100 = {power}");

    // let a = 23.to_biguint().unwrap();
    // let b = 5.to_biguint().unwrap();
    // let (res1, res2) = algorithms::eea(&a, &b);
    // println!("---------------------------");
    // println!("{res1}, {res2}");

    // let a = 29.to_biguint().unwrap();
    // if algorithms::f_test(&a) {
    //     println!("{a} is likely prime");
    // } else {
    //     println!("{a} is not prime");
    // }

    // let p = algorithms::get_prime(10);
    // println!("-------------");
    // println!("{p} might be a prime")

    for i in 0..1000 {
        let p = algorithms::get_prime(10, 5);
        if algorithms::f_test(&p, 5) {
            println!("{p}");
        } else {
            println!("{p} is not prime");
        }
    }
}
