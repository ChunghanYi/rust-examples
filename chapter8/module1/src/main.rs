mod myrandom;
use crate::myrandom::{linear, xorshift};

fn main() {
    let mut seed1 = 12345u32;
    let mut seed2 = 12345u32;

    for i in 1..10 {
        let r1 = linear::rand(&mut seed1) % 6 + 1;
        let r2 = xorshift::rand(&mut seed2) % 6 + 1;
        println!("L : {:2} 번째 = {}, {}", i+1, r1, r2);
    }
}
