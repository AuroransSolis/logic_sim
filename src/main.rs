#![allow(dead_code, unused_variables)]
#![feature(test)]

use std::rc::Rc;
use std::cell::Cell;

pub mod board;

pub(crate) use board::line::Line;
pub(crate) use board::gate::Gate;
pub(crate) use board::circuit::Circuit;
pub(crate) use board::board::Board;
pub(crate) use board::basics;

fn main() {
    println!("wewe");
}

extern crate test;

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    use super::{Gate, from_bool_8, from_u8, from_bool_3, Rc, Cell};

    #[bench]
    fn bench_128i_3c(b: &mut Bencher) {
        let mut mux = Gate::new_ns(128 + 3, 16, |inputs, outputs| {
            if inputs.contains(&None) {
                return;
            }
            for i in 0..3 {
                if inputs[i].as_ref().unwrap().get().is_none() {
                    return;
                }
            }
            let mut control_input = [false; 3];
            for i in 0..3 {
                control_input[i] = inputs[i].as_ref().unwrap().get().unwrap();
            }
            let addr = from_bool_3(&control_input);
            for i in 0..16 {
                outputs[i].set(inputs[addr * 8 + i].as_ref().unwrap().get());
            }
        });
        let mut controls = [Rc::new(Cell::new(Some(false))), Rc::new(Cell::new(Some(false))),
            Rc::new(Cell::new(Some(false)))];
        let mut inputs = Vec::new();
        for i in 0..128 {
            inputs.push(Rc::new(Cell::new(Some(false))));
        }
        for i in 0..3 {
            mux.set_input(i, Some(controls[i].clone()));
        }
        for i in 0..128 {
            mux.set_input(3 + i, Some(inputs[i].clone()));
        }
        mux.update_inputs(|inputs, outputs| {
            if !inputs.contains(&None) {
                for i in 0..16 {
                    outputs[i].set(Some(false));
                }
            }
        });
        b.iter(|| black_box(mux.eval()));
    }

    #[bench]
    fn bench_ram_8(b: &mut Bencher) {
        let mut ram_8 = Gate::new_s(18, 256 * 8, 8, |inputs, storage, outputs| {
            if inputs.contains(&None) {
                return;
            }
            for i in 0..18 {
                if inputs[i].as_ref().unwrap().get().is_none() {
                    return;
                }
            }
            let mut addr_inputs = [false; 8];
            let mut write_val = [false; 8];
            for i in 0..8 {
                addr_inputs[i] = inputs[i].as_ref().unwrap().get().unwrap();
                write_val[i] = inputs[10 + i].as_ref().unwrap().get().unwrap();
            }
            let write = inputs[8].as_ref().unwrap().get().unwrap();
            let read = inputs[9].as_ref().unwrap().get().unwrap();
            let addr = from_bool_8(&addr_inputs);
            if write {
                for i in 0..8 {
                    storage[8 * addr + i] = write_val[i];
                }
            }
            if read {
                for i in 0..8 {
                    outputs[i].set(Some(storage[8 * addr + i]));
                }
            }
        });
        let addr = [Rc::new(Cell::new(Some(false))), Rc::new(Cell::new(Some(false))),
            Rc::new(Cell::new(Some(false))), Rc::new(Cell::new(Some(false))),
            Rc::new(Cell::new(Some(false))), Rc::new(Cell::new(Some(false))),
            Rc::new(Cell::new(Some(false))), Rc::new(Cell::new(Some(false)))];
        let write = Rc::new(Cell::new(Some(false)));
        let read = Rc::new(Cell::new(Some(false)));
        let write_val = [Rc::new(Cell::new(Some(false))), Rc::new(Cell::new(Some(false))),
            Rc::new(Cell::new(Some(false))), Rc::new(Cell::new(Some(false))),
            Rc::new(Cell::new(Some(false))), Rc::new(Cell::new(Some(false))),
            Rc::new(Cell::new(Some(false))), Rc::new(Cell::new(Some(false)))];
        for i in 0..8 {
            ram_8.set_input(i, Some(addr[i].clone()));
            ram_8.set_input(10 + i, Some(write_val[i].clone()));
        }
        ram_8.set_input(8, Some(write.clone()));
        ram_8.set_input(9, Some(read.clone()));
        ram_8.update_inputs(|inputs, outputs| {
            if !inputs.contains(&None) {
                for i in 0..outputs.len() {
                    outputs[i].set(Some(false));
                }
                println!("Set all outputs to false.");
            }
        });
        read.set(Some(true));
        b.iter(|| black_box(ram_8.eval()));
    }
}

fn from_rc_cell_option_bool(other: &[Rc<Cell<Option<bool>>>; 8]) -> u8 {
    let mut total = 0;
    for i in 0..8 {
        if let Some(true) = other[i].as_ref().get() {
            total += 1 << i as u8;
        }
    }
    total
}

fn from_bool_8(other: &[bool; 8]) -> usize {
    let mut total = 0;
    for i in 0..8 {
        if other[i] {
            total += 1 << i;
        }
    }
    total
}

fn from_bool_3(other: &[bool; 3]) -> usize {
    let mut total = 0;
    for i in 0..3 {
        if other[i] {
            total += 1 << i;
        }
    }
    total
}

fn from_u8(other: u8) -> [bool; 8] {
    let mut out = [false; 8];
    for i in 0..8 {
        if other >> i & 1 == 1 {
            out[i] = true;
        }
    }
    out
}

fn from_u16(other: u16) -> [bool; 16] {
    let mut out = [false; 16];
    for i in 0..16 {
        if other >> i & 1 == 1 {
            out[i] = true;
        }
    }
    out
}

fn set_to(awv_array: &[Rc<Cell<Option<bool>>>; 8], awv: u8) {
    let new_awv_array = from_u8(awv);
    for i in 0..8 {
        awv_array[i].set(Some(new_awv_array[i]));
    }
}