#![allow(dead_code, unused_variables)]

use std::rc::Rc;
use std::cell::Cell;

pub mod board;

pub(crate) use board::gate::Gate;
pub(crate) use board::circuit::Circuit;
pub(crate) use board::board::Board;

fn main() {
    let mut board = Board::new();
    let mut c1 = Circuit::new();
    let ram_8 = Gate::new_s(18, 256 * 8, 8, |inputs, storage, outputs| {
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
            println!("Writing {:?} to address {}", write_val, addr);
            for i in 0..8 {
                storage[8 * addr + i] = write_val[i];
            }
        }
        if read {
            println!("Reading from address: {}", addr);
            for i in 0..8 {
                println!("Setting 'outputs[{}]' to 'storage[{}]' ({:?})", i, 8 * addr + i, storage[8 * addr + i]);
                outputs[i].set(Some(storage[8 * addr + i]));
                println!("'outputs[{}]' is now: {:?}", i, outputs[i]);
            }
            println!("'outputs' is now: {:?}", outputs);
        }
    });
    let ram_8 = c1.add_gate(ram_8);
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
        c1[ram_8].set_i(i, Some(addr[i].clone()));
        c1[ram_8].set_i(10 + i, Some(write_val[i].clone()));
    }
    c1[ram_8].set_i(8, Some(write.clone()));
    c1[ram_8].set_i(9, Some(read.clone()));
    c1[ram_8].update_inputs(|inputs, outputs| {
        if !inputs.contains(&None) {
            for i in 0..outputs.len() {
                outputs[i].set(Some(false));
            }
            println!("Set all outputs to false.");
        }
    });
    write.set(Some(true));
    for i in 0..=255 {
        set_to(&addr, i);
        set_to(&write_val, i);
        c1[ram_8].eval();
    }
    write.set(Some(false));
    println!("Set values at all addresses to their own address.");
    let example_address = 73;
    set_to(&addr, example_address);
    read.set(Some(true));
    c1[ram_8].eval();
    println!("Reading from example address {}:", example_address);
    for i in 0..8 {
        println!("{:?}", c1[ram_8].get_output_value(i));
    }
}

fn set_to(awv_array: &[Rc<Cell<Option<bool>>>; 8], awv: u8) {
    let new_awv_array = from_u8(awv);
    for i in 0..8 {
        awv_array[i].set(Some(new_awv_array[i]));
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

fn from_u8(other: u8) -> [bool; 8] {
    let mut out = [false; 8];
    for i in 0..8 {
        if other >> i & 1 == 1 {
            out[i] = true;
        }
    }
    out
}