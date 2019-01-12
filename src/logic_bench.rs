#[macro_use] extern crate criterion;

use criterion::{Criterion, black_box};

pub mod board;

use board::line::Line;
use board::gate::Gate;

use std::cell::Cell;
use std::rc::Rc;

fn bench_128i_3c_mux(c: &mut Criterion) {
    let mut mux = Gate::new_ns(128 + 3, 16, |inputs, outputs| {
        if !inputs[128].get().is_disconnected() && !inputs[129].get().is_disconnected()
            && !inputs[130].get().is_disconnected() {
            let select = {
                let mut total = 0;
                for i in 0..3 {
                    if inputs[128 + i].get().is_high() {
                        total += 1 << i;
                    }
                }
                total
            };
            for i in 0..16 {
                outputs[i].set(inputs[select * 16 + i].get());
            }
        }
    });
    let controls = [Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low))];
    let mut inputs: [Rc<Cell<Line>>; 128] = unsafe { std::mem::uninitialized() };
    for i in 0..128 {
        unsafe {
            (&mut inputs[i] as *mut Rc<Cell<Line>>).write(Rc::new(Cell::new(Line::Low)));
        }
    }
    for i in 0..3 {
        mux.set_input(128 + i, controls[i].clone());
    }
    for i in 0..128 {
        mux.set_input(i, inputs[i].clone());
    }
    let mut counter = 0;
    c.bench_function("MUX gate", move |b| b.iter(|| {
        let tmp = inputs[counter % 128].get();
        inputs[counter % 128].set(!tmp);
        for i in 0..3 {
            controls[i].set((counter >> i & 1 == 1).into());
        };
        mux.eval();
        counter += 1;
    }));
}

fn bench_128i_3c_mux_const(c: &mut Criterion) {
    let mut mux = Gate::new_ns(128 + 3, 16, |inputs, outputs| {
        if !inputs[128].get().is_disconnected() && !inputs[129].get().is_disconnected()
            && !inputs[130].get().is_disconnected() {
            let select = {
                let mut total = 0;
                for i in 0..3 {
                    if inputs[128 + i].get().is_high() {
                        total += 1 << i;
                    }
                }
                total
            };
            for i in 0..16 {
                outputs[i].set(inputs[select * 16 + i].get());
            }
        }
    });
    let controls = [Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low))];
    let mut inputs: [Rc<Cell<Line>>; 128] = unsafe { std::mem::uninitialized() };
    for i in 0..128 {
        unsafe {
            (&mut inputs[i] as *mut Rc<Cell<Line>>).write(Rc::new(Cell::new(Line::Low)));
        }
    }
    for i in 0..3 {
        mux.set_input(128 + i, controls[i].clone());
    }
    for i in 0..128 {
        mux.set_input(i, inputs[i].clone());
    }
    c.bench_function("MUX gate (const)", move |b| b.iter(|| black_box(mux.eval())));
}

fn bench_ram_8(c: &mut Criterion) {
    let mut ram_8 = Gate::nor_latch_ram_8();
    let addr = [Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low))];
    let write = Rc::new(Cell::new(Line::High));
    let read = Rc::new(Cell::new(Line::High));
    let clock = Rc::new(Cell::new(Line::High));
    let write_val = [Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low))];
    for i in 0..8 {
        ram_8.set_input(i, addr[i].clone());
        ram_8.set_input(8 + i, write_val[i].clone());
    }
    ram_8.set_input(16, write.clone());
    ram_8.set_input(17, read.clone());
    ram_8.set_input(18, clock.clone());
    let mut counter = 0;
    c.bench_function("Memory module WHRHCH", move |b| b.iter(|| {
        let tmp = addr[counter % 8].get();
        addr[counter % 8].set(!tmp);
        let tmp = write_val[7 - (counter % 8)].get();
        write_val[7 - (counter % 8)].set(!tmp);
        write.set(Line::High);
        ram_8.eval();
        write.set(Line::Low);
        read.set(Line::High);
        ram_8.eval();
        read.set(Line::Low);
        ram_8.eval();
        counter += 1;
    }));
}

fn bench_ram_8_const(c: &mut Criterion) {
    let mut ram_8 = Gate::nor_latch_ram_8();
    let addr = [Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low))];
    let write = Rc::new(Cell::new(Line::High));
    let read = Rc::new(Cell::new(Line::High));
    let clock = Rc::new(Cell::new(Line::High));
    let write_val = [Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low))];
    for i in 0..8 {
        ram_8.set_input(i, addr[i].clone());
        ram_8.set_input(8 + i, write_val[i].clone());
    }
    ram_8.set_input(16, write.clone());
    ram_8.set_input(17, read.clone());
    ram_8.set_input(18, clock.clone());
    c.bench_function("Memory module WHRHCH (const)", move |b| b.iter(|| black_box(ram_8.eval())));
}

macro_rules! make_16_wide_gate_and_set_inputs {
    ($and_gate:ident, $inputs:ident, $block_number:expr) => {
        make_16_wide_gate($and_gate.get_output(0), $inputs[$block_number * 16].clone(),
            $inputs[$block_number * 16 + 1].clone(),
            $inputs[$block_number * 16 + 2].clone(),
            $inputs[$block_number * 16 + 3].clone(),
            $inputs[$block_number * 16 + 4].clone(),
            $inputs[$block_number * 16 + 5].clone(),
            $inputs[$block_number * 16 + 6].clone(),
            $inputs[$block_number * 16 + 7].clone(),
            $inputs[$block_number * 16 + 8].clone(),
            $inputs[$block_number * 16 + 9].clone(),
            $inputs[$block_number * 16 + 10].clone(),
            $inputs[$block_number * 16 + 11].clone(),
            $inputs[$block_number * 16 + 12].clone(),
            $inputs[$block_number * 16 + 13].clone(),
            $inputs[$block_number * 16 + 14].clone(),
            $inputs[$block_number * 16 + 15].clone())
    };
}

macro_rules! push {
    ($vec:ident: $($gate:ident),+) => {
        $(
            $vec.push($gate);
        )+
    }
}

fn bench_128i_3c_mux_gates(c: &mut Criterion) {
    let controls = [Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low))];
    let mut inputs = Vec::new();
    for _ in 0..128 {
        inputs.push(Rc::new(Cell::new(Line::Low)));
    }
    let mut mux_gates = Vec::new();
    let mut output_locations = Vec::new();
    mux_gates.push(Gate::high_source());
    let mut and_017 = Gate::and();
    unsafe { and_017.set_inputs(&[controls[0].clone(), controls[1].clone()]) };
    let mut and_237 = Gate::and();
    unsafe { and_237.set_inputs(&[controls[2].clone(), mux_gates[0].get_output(0)]) };
    let mut and_7 = Gate::and();
    unsafe { and_7.set_inputs(&[and_017.get_output(0), and_237.get_output(0)]) };
    let mut gate_7 = make_16_wide_gate_and_set_inputs!(and_7, inputs, 7);
    push!(mux_gates: and_017, and_237, and_7);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_7);
    let mut not_06 = Gate::not();
    not_06.set_input(0, controls[0].clone());
    let mut and_016 = Gate::and();
    unsafe { and_016.set_inputs(&[not_06.get_output(0), controls[1].clone()]) };
    let mut and_236 = Gate::and();
    unsafe { and_236.set_inputs(&[controls[2].clone(), mux_gates[0].get_output(0)]) };
    let mut and_6 = Gate::and();
    unsafe { and_6.set_inputs(&[and_016.get_output(0), and_236.get_output(0)]) };
    let mut gate_6 = make_16_wide_gate_and_set_inputs!(and_6, inputs, 6);
    push!(mux_gates: not_06, and_016, and_236, and_6);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_6);
    let mut not_15 = Gate::not();
    not_15.set_input(0, controls[1].clone());
    let mut and_015 = Gate::and();
    unsafe { and_015.set_inputs(&[controls[0].clone(), not_15.get_output(0)]) };
    let mut and_235 = Gate::and();
    unsafe { and_235.set_inputs(&[controls[2].clone(), mux_gates[0].get_output(0)]) };
    let mut and_5 = Gate::and();
    unsafe { and_5.set_inputs(&[and_015.get_output(0), and_235.get_output(0)]) };
    let mut gate_5 = make_16_wide_gate_and_set_inputs!(and_5, inputs, 5);
    push!(mux_gates: not_15, and_015, and_235, and_5);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_5);
    let mut not_04 = Gate::and();
    not_04.set_input(0, controls[0].clone());
    let mut not_14 = Gate::and();
    not_14.set_input(0, controls[1].clone());
    let mut and_014 = Gate::and();
    unsafe { and_014.set_inputs(&[not_04.get_output(0), not_14.get_output(0)]) };
    let mut and_234 = Gate::and();
    unsafe { and_234.set_inputs(&[controls[2].clone(), mux_gates[0].get_output(0)]) };
    let mut and_4 = Gate::and();
    unsafe { and_4.set_inputs(&[and_014.get_output(0), and_234.get_output(0)]) };
    let mut gate_4 = make_16_wide_gate_and_set_inputs!(and_4, inputs, 4);
    push!(mux_gates: not_04, not_14, and_014, and_234, and_4);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_4);
    let mut not_23 = Gate::not();
    not_23.set_input(0, controls[2].clone());
    let mut and_013 = Gate::and();
    unsafe { and_013.set_inputs(&[controls[0].clone(), controls[1].clone()]) };
    let mut and_233 = Gate::and();
    unsafe { and_233.set_inputs(&[not_23.get_output(0), mux_gates[0].get_output(0)]) };
    let mut and_3 = Gate::and();
    unsafe { and_3.set_inputs(&[and_013.get_output(0), and_233.get_output(0)]) };
    let mut gate_3 = make_16_wide_gate_and_set_inputs!(and_3, inputs, 3);
    push!(mux_gates: not_23, and_013, and_233, and_3);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_3);
    let mut not_02 = Gate::not();
    not_02.set_input(0, controls[0].clone());
    let mut not_22 = Gate::not();
    not_22.set_input(0, controls[2].clone());
    let mut and_012 = Gate::and();
    unsafe { and_012.set_inputs(&[not_02.get_output(0), controls[1].clone()]) };
    let mut and_232 = Gate::and();
    unsafe { and_232.set_inputs(&[not_22.get_output(0), mux_gates[0].get_output(0)]) };
    let mut and_2 = Gate::and();
    unsafe { and_2.set_inputs(&[and_012.get_output(0), and_232.get_output(0)]) };
    let mut gate_2 = make_16_wide_gate_and_set_inputs!(and_2, inputs, 2);
    push!(mux_gates: not_02, not_22, and_012, and_232, and_2);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_2);
    let mut not_11 = Gate::not();
    not_11.set_input(0, controls[1].clone());
    let mut not_21 = Gate::not();
    not_21.set_input(0, controls[2].clone());
    let mut and_011 = Gate::and();
    unsafe { and_011.set_inputs(&[controls[0].clone(), not_11.get_output(0)]) };
    let mut and_231 = Gate::and();
    unsafe { and_231.set_inputs(&[not_21.get_output(0), mux_gates[0].get_output(0)]) };
    let mut and_1 = Gate::and();
    unsafe { and_1.set_inputs(&[and_011.get_output(0), and_231.get_output(0)]) };
    let mut gate_1 = make_16_wide_gate_and_set_inputs!(and_1, inputs, 1);
    push!(mux_gates: not_11, not_21, and_011, and_231, and_1);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_1);
    let mut not_00 = Gate::not();
    not_00.set_input(0, controls[0].clone());
    let mut not_10 = Gate::not();
    not_10.set_input(0, controls[1].clone());
    let mut not_20 = Gate::not();
    not_20.set_input(0, controls[2].clone());
    let mut and_010 = Gate::and();
    unsafe { and_010.set_inputs(&[not_00.get_output(0), not_10.get_output(0)]) };
    let mut and_230 = Gate::and();
    unsafe { and_230.set_inputs(&[not_20.get_output(0), mux_gates[0].get_output(0)]) };
    let mut and_0 = Gate::and();
    unsafe { and_0.set_inputs(&[and_010.get_output(0), and_230.get_output(0)]) };
    let mut gate_0 = make_16_wide_gate_and_set_inputs!(and_0, inputs, 0);
    push!(mux_gates: not_00, not_10, not_20, and_010, and_230, and_0);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_0);
    for i in 0..16 {
        let rc_lines = (0..8).map(|j| mux_gates[i + 16 * j].get_output(0)).collect::<Vec<_>>();
        mux_gates.append(&mut make_8_way_or(&rc_lines));
    }
    let mut counter = 0;
    c.bench_function("MUX gate of `Gate`s", move |b| b.iter(|| {
        let tmp = inputs[counter % 128].get();
        inputs[counter % 128].set(!tmp);
        for i in 0..3 {
            controls[i].set((counter >> i & 1 == 1).into());
        };
        for gate in &mut mux_gates {
            gate.eval();
        }
        counter += 1;
    }));
}

fn bench_128i_3c_mux_gates_const(c: &mut Criterion) {
    let controls = [Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low))];
    let mut inputs = Vec::new();
    for _ in 0..128 {
        inputs.push(Rc::new(Cell::new(Line::Low)));
    }
    let mut mux_gates = Vec::new();
    let mut output_locations = Vec::new();
    mux_gates.push(Gate::high_source());
    let mut and_017 = Gate::and();
    unsafe { and_017.set_inputs(&[controls[0].clone(), controls[1].clone()]) };
    let mut and_237 = Gate::and();
    unsafe { and_237.set_inputs(&[controls[2].clone(), mux_gates[0].get_output(0)]) };
    let mut and_7 = Gate::and();
    unsafe { and_7.set_inputs(&[and_017.get_output(0), and_237.get_output(0)]) };
    let mut gate_7 = make_16_wide_gate_and_set_inputs!(and_7, inputs, 7);
    push!(mux_gates: and_017, and_237, and_7);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_7);
    let mut not_06 = Gate::not();
    not_06.set_input(0, controls[0].clone());
    let mut and_016 = Gate::and();
    unsafe { and_016.set_inputs(&[not_06.get_output(0), controls[1].clone()]) };
    let mut and_236 = Gate::and();
    unsafe { and_236.set_inputs(&[controls[2].clone(), mux_gates[0].get_output(0)]) };
    let mut and_6 = Gate::and();
    unsafe { and_6.set_inputs(&[and_016.get_output(0), and_236.get_output(0)]) };
    let mut gate_6 = make_16_wide_gate_and_set_inputs!(and_6, inputs, 6);
    push!(mux_gates: not_06, and_016, and_236, and_6);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_6);
    let mut not_15 = Gate::not();
    not_15.set_input(0, controls[1].clone());
    let mut and_015 = Gate::and();
    unsafe { and_015.set_inputs(&[controls[0].clone(), not_15.get_output(0)]) };
    let mut and_235 = Gate::and();
    unsafe { and_235.set_inputs(&[controls[2].clone(), mux_gates[0].get_output(0)]) };
    let mut and_5 = Gate::and();
    unsafe { and_5.set_inputs(&[and_015.get_output(0), and_235.get_output(0)]) };
    let mut gate_5 = make_16_wide_gate_and_set_inputs!(and_5, inputs, 5);
    push!(mux_gates: not_15, and_015, and_235, and_5);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_5);
    let mut not_04 = Gate::and();
    not_04.set_input(0, controls[0].clone());
    let mut not_14 = Gate::and();
    not_14.set_input(0, controls[1].clone());
    let mut and_014 = Gate::and();
    unsafe { and_014.set_inputs(&[not_04.get_output(0), not_14.get_output(0)]) };
    let mut and_234 = Gate::and();
    unsafe { and_234.set_inputs(&[controls[2].clone(), mux_gates[0].get_output(0)]) };
    let mut and_4 = Gate::and();
    unsafe { and_4.set_inputs(&[and_014.get_output(0), and_234.get_output(0)]) };
    let mut gate_4 = make_16_wide_gate_and_set_inputs!(and_4, inputs, 4);
    push!(mux_gates: not_04, not_14, and_014, and_234, and_4);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_4);
    let mut not_23 = Gate::not();
    not_23.set_input(0, controls[2].clone());
    let mut and_013 = Gate::and();
    unsafe { and_013.set_inputs(&[controls[0].clone(), controls[1].clone()]) };
    let mut and_233 = Gate::and();
    unsafe { and_233.set_inputs(&[not_23.get_output(0), mux_gates[0].get_output(0)]) };
    let mut and_3 = Gate::and();
    unsafe { and_3.set_inputs(&[and_013.get_output(0), and_233.get_output(0)]) };
    let mut gate_3 = make_16_wide_gate_and_set_inputs!(and_3, inputs, 3);
    push!(mux_gates: not_23, and_013, and_233, and_3);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_3);
    let mut not_02 = Gate::not();
    not_02.set_input(0, controls[0].clone());
    let mut not_22 = Gate::not();
    not_22.set_input(0, controls[2].clone());
    let mut and_012 = Gate::and();
    unsafe { and_012.set_inputs(&[not_02.get_output(0), controls[1].clone()]) };
    let mut and_232 = Gate::and();
    unsafe { and_232.set_inputs(&[not_22.get_output(0), mux_gates[0].get_output(0)]) };
    let mut and_2 = Gate::and();
    unsafe { and_2.set_inputs(&[and_012.get_output(0), and_232.get_output(0)]) };
    let mut gate_2 = make_16_wide_gate_and_set_inputs!(and_2, inputs, 2);
    push!(mux_gates: not_02, not_22, and_012, and_232, and_2);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_2);
    let mut not_11 = Gate::not();
    not_11.set_input(0, controls[1].clone());
    let mut not_21 = Gate::not();
    not_21.set_input(0, controls[2].clone());
    let mut and_011 = Gate::and();
    unsafe { and_011.set_inputs(&[controls[0].clone(), not_11.get_output(0)]) };
    let mut and_231 = Gate::and();
    unsafe { and_231.set_inputs(&[not_21.get_output(0), mux_gates[0].get_output(0)]) };
    let mut and_1 = Gate::and();
    unsafe { and_1.set_inputs(&[and_011.get_output(0), and_231.get_output(0)]) };
    let mut gate_1 = make_16_wide_gate_and_set_inputs!(and_1, inputs, 1);
    push!(mux_gates: not_11, not_21, and_011, and_231, and_1);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_1);
    let mut not_00 = Gate::not();
    not_00.set_input(0, controls[0].clone());
    let mut not_10 = Gate::not();
    not_10.set_input(0, controls[1].clone());
    let mut not_20 = Gate::not();
    not_20.set_input(0, controls[2].clone());
    let mut and_010 = Gate::and();
    unsafe { and_010.set_inputs(&[not_00.get_output(0), not_10.get_output(0)]) };
    let mut and_230 = Gate::and();
    unsafe { and_230.set_inputs(&[not_20.get_output(0), mux_gates[0].get_output(0)]) };
    let mut and_0 = Gate::and();
    unsafe { and_0.set_inputs(&[and_010.get_output(0), and_230.get_output(0)]) };
    let mut gate_0 = make_16_wide_gate_and_set_inputs!(and_0, inputs, 0);
    push!(mux_gates: not_00, not_10, not_20, and_010, and_230, and_0);
    (0..16).for_each(|i| output_locations.push(mux_gates.len() + i));
    mux_gates.append(&mut gate_0);
    for i in 0..16 {
        let rc_lines = (0..8).map(|j| mux_gates[i + 16 * j].get_output(0)).collect::<Vec<_>>();
        mux_gates.append(&mut make_8_way_or(&rc_lines));
    }
    c.bench_function("MUX gate of `Gate`s (const)", move |b| b.iter(|| {
        for gate in &mut mux_gates {
            black_box(gate.eval());
        }
    }));
}

fn bench_128i_3c_mux_conditionless(c: &mut Criterion) {
    let mut mux = Gate::new_ns(131, 16, |inputs, outputs| {
        let gates = [
            !inputs[128].get() & !inputs[129].get() & !inputs[130].get(),
            !inputs[128].get() & !inputs[129].get() &  inputs[130].get(),
            !inputs[128].get() &  inputs[129].get() & !inputs[130].get(),
            !inputs[128].get() &  inputs[129].get() &  inputs[130].get(),
             inputs[128].get() & !inputs[129].get() & !inputs[130].get(),
             inputs[128].get() & !inputs[129].get() &  inputs[130].get(),
             inputs[128].get() &  inputs[129].get() & !inputs[130].get(),
             inputs[128].get() &  inputs[129].get() &  inputs[130].get()
        ];
        for i in 0..16 {
            for j in 0..8 {
                let n = inputs[i + j * 16].get() & gates[j] | outputs[i].get();
                outputs[i].set(n);
            }
        }
    });
    let controls = [Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low))];
    let mut inputs: [Rc<Cell<Line>>; 128] = unsafe { std::mem::uninitialized() };
    for i in 0..128 {
        unsafe {
            (&mut inputs[i] as *mut Rc<Cell<Line>>).write(Rc::new(Cell::new(Line::Low)));
        }
    }
    for i in 0..3 {
        mux.set_input(128 + i, controls[i].clone());
    }
    for i in 0..128 {
        mux.set_input(i, inputs[i].clone());
    }
    let mut counter = 0;
    c.bench_function("MUX gate without conditionals", move |b| b.iter(|| {
        let tmp = inputs[counter % 128].get();
        inputs[counter % 128].set(!tmp);
        for i in 0..3 {
            controls[i].set((counter >> i & 1 == 1).into());
        };
        mux.eval();
        counter += 1;
    }));
}

fn bench_128i_3c_mux_conditionless_const(c: &mut Criterion) {
    let mut mux = Gate::new_ns(131, 16, |inputs, outputs| {
        let gates = [
            !inputs[128].get() & !inputs[129].get() & !inputs[130].get(),
            !inputs[128].get() & !inputs[129].get() &  inputs[130].get(),
            !inputs[128].get() &  inputs[129].get() & !inputs[130].get(),
            !inputs[128].get() &  inputs[129].get() &  inputs[130].get(),
            inputs[128].get() & !inputs[129].get() & !inputs[130].get(),
            inputs[128].get() & !inputs[129].get() &  inputs[130].get(),
            inputs[128].get() &  inputs[129].get() & !inputs[130].get(),
            inputs[128].get() &  inputs[129].get() &  inputs[130].get()
        ];
        for i in 0..16 {
            for j in 0..8 {
                let n = inputs[i + j * 16].get() & gates[j] | outputs[i].get();
                outputs[i].set(n);
            }
        }
    });
    let controls = [Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low))];
    let mut inputs: [Rc<Cell<Line>>; 128] = unsafe { std::mem::uninitialized() };
    for i in 0..128 {
        unsafe {
            (&mut inputs[i] as *mut Rc<Cell<Line>>).write(Rc::new(Cell::new(Line::Low)));
        }
    }
    for i in 0..3 {
        mux.set_input(128 + i, controls[i].clone());
    }
    for i in 0..128 {
        mux.set_input(i, inputs[i].clone());
    }
    c.bench_function("MUX gate without conditionals (const)", move |b| b.iter(|| {
        black_box(mux.eval());
    }));
}

fn bench_get_input(c: &mut Criterion) {
    let gate = Gate::master_slave_flip_flop_ram_16();
    c.bench_function("Get input (as_ptr())", move |b| b.iter(|| black_box(gate.get_input(17))));
}

use std::time::Duration;

criterion_group!{
    name = logic_benches;
    config = Criterion::default().sample_size(10_000).measurement_time(Duration::from_secs(60));
    targets = bench_128i_3c_mux, bench_128i_3c_mux_const, bench_ram_8, bench_ram_8_const,
        bench_128i_3c_mux_gates, bench_128i_3c_mux_gates_const, bench_128i_3c_mux_conditionless,
        bench_128i_3c_mux_conditionless_const
}
criterion_main!{logic_benches}

macro_rules! attach_toggle_and_lines {
    ($gate_vec:ident $toggle:ident: $($line:ident),+) => {
        $(
            $gate_vec.push({
                let mut and = Gate::and();
                and.set_input(0, $toggle.clone());
                and.set_input(1, $line);
                and
            });
        )*
    }
}

fn make_16_wide_gate(toggle: Rc<Cell<Line>>, l0: Rc<Cell<Line>>, l1: Rc<Cell<Line>>,
    l2: Rc<Cell<Line>>, l3: Rc<Cell<Line>>, l4: Rc<Cell<Line>>, l5: Rc<Cell<Line>>,
    l6: Rc<Cell<Line>>, l7: Rc<Cell<Line>>, l8: Rc<Cell<Line>>, l9: Rc<Cell<Line>>,
    la: Rc<Cell<Line>>, lb: Rc<Cell<Line>>, lc: Rc<Cell<Line>>, ld: Rc<Cell<Line>>,
    le: Rc<Cell<Line>>, lf: Rc<Cell<Line>>) -> Vec<Gate> {
    let mut gates = Vec::with_capacity(16);
    attach_toggle_and_lines!(gates toggle: l0, l1, l2, l3, l4, l5, l6, l7, l8, l9, la, lb, lc,
        ld, le, lf);
    gates
}

fn make_8_way_or(inputs: &[Rc<Cell<Line>>]) -> Vec<Gate> {
    let mut gates = Vec::new();
    let mut or_01 = Gate::or();
    unsafe { or_01.set_inputs(&inputs[0..2]) };
    let mut or_23 = Gate::or();
    unsafe { or_23.set_inputs(&inputs[2..4]) };
    let mut or_45 = Gate::or();
    unsafe { or_45.set_inputs(&inputs[4..6]) };
    let mut or_67 = Gate::or();
    unsafe { or_67.set_inputs(&inputs[6..8]) };
    let mut or_0123 = Gate::or();
    unsafe { or_0123.set_inputs(&[or_01.get_input(0), or_23.get_input(0)]) };
    let mut or_4567 = Gate::or();
    unsafe { or_4567.set_inputs(&[or_45.get_output(0), or_67.get_output(0)]) };
    let mut or_01234567 = Gate::or();
    unsafe { or_01234567.set_inputs(&[or_0123.get_output(0), or_4567.get_output(0)]) };
    push!(gates: or_01, or_23, or_45, or_67, or_0123, or_4567, or_01234567);
    gates
}