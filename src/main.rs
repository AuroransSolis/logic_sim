#![allow(dead_code, unused_variables)]

pub mod board;

pub(crate) use board::gate::Gate;
pub(crate) use board::circuit::Circuit;
pub(crate) use board::board::Board;

fn main() {
    let mut board = Board::new();
    let mut c1 = Circuit::new();
    let mut gate_ram_8 = Gate::new_s(18, 256 * 8, 8, |inputs, storage, outputs| {
        if inputs.iter().any(|i| i.is_none()) {
            return;
        }
        let mut addr_inputs = [None; 8];
        for i in 0..8 {
            if let Some(ref rcoe) = inputs[i] {
                addr_inputs[i] = rcoe.get();
            }
        }
        if addr_inputs.iter().any(|addr_input| addr_input.is_none()) {
            return;
        }
        let write = inputs[8].unwrap().get();
        let read = inputs[9].unwrap().get();

    });
}