#![allow(dead_code, unused_variables)]

#[macro_use] pub(crate) mod gate;

use self::gate::Gate::*;

fn main() {
    let high_source = new_gate!(SOURCE true);
    let low_source = new_gate!(SOURCE false);
    let gate_1 = new_gate!(AND Some(&high_source), Some(&low_source)); // 0
    let gate_2 = new_gate!(XOR Some(&high_source), Some(&gate_1)); // 1
    let gate_3 = new_gate!(NOR Some(&gate_1), Some(&gate_2)); // 0
    let gate_1 = gate_1.eval();
    let gate_2 = gate_2.eval();
    let gate_3 = gate_3.eval();
    println!("Gate 1: {:?}", gate_1.get_output());
    println!("Gate 2: {:?}", gate_2.get_output());
    println!("Gate 3: {:?}", gate_3.get_output());
}