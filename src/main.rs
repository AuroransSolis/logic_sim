#![allow(dead_code, unused_variables)]

use std::rc::Rc;
use std::cell::Cell;

pub mod board;

pub(crate) use board::line::Line;
pub(crate) use board::gate::Gate;
//pub(crate) use board::circuit::Circuit;
//pub(crate) use board::board::Board;

fn main() {
    test_128i_3c_mux_gates();
}

fn test_128i_3c_mux_gates() {
    let controls = [Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low))];
    let mut inputs = Vec::new();
    for _ in 0..128 {
        inputs.push(Rc::new(Cell::new(Line::Low)));
    }
    let mut mux_gates = Vec::new();
    for i in 0..16 {
        mux_gates.append(&mut make_8_way_mux([inputs[8 * i].clone(), inputs[8 * i + 1].clone(),
            inputs[8 * i + 2].clone(), inputs[8 * i + 3].clone(), inputs[8 * i + 4].clone(),
            inputs[8 * i + 5].clone(), inputs[8 * i + 6].clone(), inputs[8 * i + 7].clone()],
            controls[2].clone(), controls[1].clone(), controls[0].clone()));
    }
    println!("{}", mux_gates.len());
}

macro_rules! push {
    ($vec:ident: $($gate:ident),+) => {
        $(
            $vec.push($gate);
        )+
    }
}

fn make_8_way_mux(inputs: [Rc<Cell<Line>>; 8], s0: Rc<Cell<Line>>, s1: Rc<Cell<Line>>,
    s2: Rc<Cell<Line>>) -> Vec<Gate> {
    let mut gates = Vec::new();
    let mut mux_00 = Gate::mux_1b_2w();
    mux_00.set_input(0, inputs[0].clone());
    mux_00.set_input(1, inputs[1].clone());
    mux_00.set_input(2, s0.clone());
    let mut mux_01 = Gate::mux_1b_2w();
    mux_01.set_input(0, inputs[2].clone());
    mux_01.set_input(1, inputs[3].clone());
    mux_01.set_input(2, s0.clone());
    let mut mux_02 = Gate::mux_1b_2w();
    mux_02.set_input(0, inputs[4].clone());
    mux_02.set_input(1, inputs[5].clone());
    mux_02.set_input(2, s0.clone());
    let mut mux_03 = Gate::mux_1b_2w();
    mux_03.set_input(0, inputs[6].clone());
    mux_03.set_input(1, inputs[7].clone());
    mux_03.set_input(2, s0.clone());
    let mut mux_10 = Gate::mux_1b_2w();
    mux_10.set_input(0, mux_00.get_output(0).clone());
    mux_10.set_input(1, mux_01.get_output(0).clone());
    mux_10.set_input(2, s1.clone());
    let mut mux_11 = Gate::mux_1b_2w();
    mux_11.set_input(0, mux_02.get_output(0).clone());
    mux_11.set_input(1, mux_03.get_output(0).clone());
    mux_11.set_input(2, s1.clone());
    let mut mux_20 = Gate::mux_1b_2w();
    mux_20.set_input(0, mux_10.get_output(0).clone());
    mux_20.set_input(1, mux_11.get_output(0).clone());
    mux_20.set_input(2, s2.clone());
    push!{gates: mux_00, mux_01, mux_02, mux_03, mux_10, mux_11, mux_20};
    gates
}