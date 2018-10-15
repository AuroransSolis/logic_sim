#![allow(dead_code, unused_variables)]

pub mod gate;
pub mod circuit;

pub(crate) use self::gate::Gate;
pub(crate) use self::circuit::Circuit;

fn main() {
    let mut circuit = Circuit::new();
    circuit.add_gate(Gate::new_source());
    circuit.set_high(0);
    circuit.add_gate(Gate::new_source());
    circuit.add_gate(Gate::new_and());
    circuit.connect_i1(2, 0);
    circuit.connect_i2(2, 1);
    circuit.add_gate(Gate::new_xor());
    circuit.connect_i1(3, 0);
    circuit.connect_i2(3, 2);
    circuit.add_gate(Gate::new_nor());
    circuit.connect_i1(4, 2);
    circuit.connect_i2(4, 3);
    circuit.eval();
    println!("Gate 1: {:?}", circuit.get_output(2));
    println!("Gate 2: {:?}", circuit.get_output(3));
    println!("Gate 3: {:?}", circuit.get_output(4));
}