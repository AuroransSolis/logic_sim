#![allow(dead_code, unused_variables)]

pub mod gate;
pub mod circuit;

pub(crate) use self::gate::Gate;
pub(crate) use self::circuit::Circuit;

fn main() {
    let mut circuit = Circuit::new();
    let source_1 = circuit.add_gate(Gate::new_source());
    let source_2 = circuit.add_gate(Gate::new_source());
    let gate_1 = circuit.add_gate(Gate::new_nor());
    circuit.connect_i1(gate_1, source_1);
    let gate_2 = circuit.add_gate(Gate::new_nor());
    circuit.connect_i1(gate_2, source_2);
    circuit.connect_i2(gate_1, gate_2);
    circuit.connect_i2(gate_2, gate_1);
    circuit.update_all_inputs();
    circuit.eval_all();
    println!("Starting state:");
    println!("(1) Gate 1: {:?}", circuit.get_output(gate_1));
    println!("(1) Gate 2: {:?}", circuit.get_output(gate_2));
    circuit.set_high(source_2);
    circuit.eval_all();
    println!("After setting source 2 high:");
    println!("(2) Gate 1: {:?}", circuit.get_output(gate_1));
    println!("(2) Gate 2: {:?}", circuit.get_output(gate_2));
    circuit.set_low(source_2);
    circuit.eval_all();
    println!("After setting source 2 low:");
    println!("(3) Gate 1: {:?}", circuit.get_output(gate_1));
    println!("(5) Gate 2: {:?}", circuit.get_output(gate_2));
    circuit.set_high(source_1);
    circuit.eval_all();
    println!("After setting source 1 high:");
    println!("(2) Gate 1: {:?}", circuit.get_output(gate_1));
    println!("(2) Gate 2: {:?}", circuit.get_output(gate_2));
    circuit.set_low(source_1);
    circuit.eval_all();
    println!("After setting source 1 low:");
    println!("(3) Gate 1: {:?}", circuit.get_output(gate_1));
    println!("(3) Gate 2: {:?}", circuit.get_output(gate_2));
}