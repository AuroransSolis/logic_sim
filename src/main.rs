#![allow(dead_code, unused_variables)]

#[macro_use] pub(crate) mod gate;

use self::gate::Gate::*;

fn main() {
    let high_source = new_gate!(SOURCE true);
    let low_source = new_gate!(SOURCE false);
    let mut gate_1 = new_gate!(AND Some(&high_source), Some(&low_source));
    gate_1.eval();
    println!("{:?}", gate_1.get_output());
}