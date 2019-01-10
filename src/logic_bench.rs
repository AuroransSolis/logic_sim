#[macro_use] extern crate criterion;

use criterion::Criterion;

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
    let mut controls = [Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
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
        let tmp = [Rc::new(Cell::new((counter & 1 == 1).into())),
            Rc::new(Cell::new((counter & 2 == 2).into())),
            Rc::new(Cell::new((counter & 4 == 4).into()))];
        controls[0..3].clone_from_slice(&tmp);
        mux.eval();
        counter += 1;
    }));
}

fn bench_ram_8(c: &mut Criterion) {
    let mut ram_8 = Gate::nor_latch_ram_8();
    let addr = [Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low))];
    let write = Rc::new(Cell::new(Line::Low));
    let read = Rc::new(Cell::new(Line::Low));
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
    c.bench_function("Memory module", move |b| b.iter(|| {
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

macro_rules! set_inputs {
    ($gate:ident, $i0:expr, $i1:expr) => {
        $gate.set_input(0, $i0);
        $gate.set_input(1, $i1);
    }
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
    let mut controls = [Rc::new(Cell::new(Line::Low)), Rc::new(Cell::new(Line::Low)),
        Rc::new(Cell::new(Line::Low))];
    let mut inputs: [Rc<Cell<Line>>; 128] = unsafe { std::mem::uninitialized() };
    for i in 0..128 {
        unsafe {
            (&mut inputs[i] as *mut Rc<Cell<Line>>).write(Rc::new(Cell::new(Line::Low)));
        }
    }
    let mut mux_gates = Vec::new();
    mux_gates.push(Gate::high_source());
    let mut and_017 = Gate::and();
    set_inputs!(and_017, controls[0].clone(), controls[1].clone());
    let mut and_237 = Gate::and();
    set_inputs!(and_237, controls[2].clone(), mux_gates[0].get_output(0));
    let mut and_7 = Gate::and();
    set_inputs!(and_7, and_017.get_output(0), and_237.get_output(0));
    let mut gate_7 = make_16_wide_gate_and_set_inputs!(and_7, inputs, 7);
    push!(mux_gates: and_017, and_237, and_7);
    mux_gates.append(&mut gate_7);
    let mut not_06 = Gate::not();
    not_06.set_input(0, controls[0].clone());
    let mut and_016 = Gate::and();
    set_inputs!(and_016, not_06.get_output(0), controls[1].clone());
    let mut and_236 = Gate::and();
    set_inputs!(and_236, controls[2].clone(), mux_gates[0].get_output(0));
    let mut and_6 = Gate::and();
    set_inputs!(and_6, and_016.get_output(0), and_236.get_output(0));
    let mut gate_6 = make_16_wide_gate_and_set_inputs!(and_6, inputs, 6);
    push!(mux_gates: not_06, and_016, and_236, and_6);
    mux_gates.append(&mut gate_6);
    let mut not_15 = Gate::not();
    not_15.set_input(0, controls[1].clone());
    let mut and_015 = Gate::and();
    set_inputs!(and_015, controls[0].clone(), not_15.get_output(0));
    let mut and_235 = Gate::and();
    set_inputs!(and_235, controls[2].clone(), mux_gates[0].get_output(0));
    let mut and_5 = Gate::and();
    set_inputs!(and_5, and_015.get_output(0), and_235.get_output(0));
    let mut gate_5 = make_16_wide_gate_and_set_inputs!(and_5, inputs, 5);
    push!(mux_gates: not_15, and_015, and_235, and_5);
    mux_gates.append(&mut gate_5);
    let mut not_04 = Gate::and();
    not_04.set_input(0, controls[0].clone());
    let mut not_14 = Gate::and();
    not_14.set_input(0, controls[1].clone());
    let mut and_014 = Gate::and();
    set_inputs!(and_014, not_04.get_output(0), not_14.get_output(0));
    let mut and_234 = Gate::and();
    set_inputs!(and_234, controls[2].clone(), mux_gates[0].get_output(0));
    let mut and_4 = Gate::and();
    set_inputs!(and_4, and_014.get_output(0), and_234.get_output(0));
    let mut gate_4 = make_16_wide_gate_and_set_inputs!(and_4, inputs, 4);
    push!(mux_gates: not_04, not_14, and_014, and_234, and_4);
    mux_gates.append(&mut gate_4);
    let mut not_23 = Gate::not();
    not_23.set_input(0, controls[2].clone());
    let mut and_013 = Gate::and();
    set_inputs!(and_013, controls[0].clone(), controls[1].clone());
    let mut and_233 = Gate::and();
    set_inputs!(and_233, not_23.get_output(0), mux_gates[0].get_output(0));
    let mut and_3 = Gate::and();
    set_inputs!(and_3, and_013.get_output(0), and_233.get_output(0));
    let mut gate_3 = make_16_wide_gate_and_set_inputs!(and_3, inputs, 3);
    push!(mux_gates: not_23, and_013, and_233, and_3);
    mux_gates.append(&mut gate_3);
    let mut not_02 = Gate::not();
    not_02.set_input(0, controls[0].clone());
    let mut not_22 = Gate::not();
    not_22.set_input(0, controls[2].clone());
    let mut and_012 = Gate::and();
    set_inputs!(and_012, not_02.get_output(0), controls[1].clone());
    let mut and_232 = Gate::and();
    set_inputs!(and_232, not_22.get_output(0), mux_gates[0].get_output(0));
    let mut and_2 = Gate::and();
    set_inputs!(and_2, and_012.get_output(0), and_232.get_output(0));
    let mut gate_2 = make_16_wide_gate_and_set_inputs!(and_2, inputs, 2);
    push!(mux_gates: not_02, not_22, and_012, and_232, and_2);
    mux_gates.append(&mut gate_2);
    let mut not_11 = Gate::not();
    not_11.set_input(0, controls[1].clone());
    let mut not_21 = Gate::not();
    not_21.set_input(0, controls[2].clone());
    let mut and_011 = Gate::and();
    set_inputs!(and_011, controls[0].clone(), not_11.get_output(0));
    let mut and_231 = Gate::and();
    set_inputs!(and_231, not_21.get_output(0), mux_gates[0].get_output(0));
    let mut and_1 = Gate::and();
    set_inputs!(and_1, and_011.get_output(0), and_231.get_output(0));
    let mut gate_1 = make_16_wide_gate_and_set_inputs!(and_1, inputs, 1);
    push!(mux_gates: not_11, not_21, and_011, and_231, and_1);
    mux_gates.append(&mut gate_1);
    let mut not_00 = Gate::not();
    not_00.set_input(0, controls[0].clone());
    let mut not_10 = Gate::not();
    not_10.set_input(0, controls[1].clone());
    let mut not_20 = Gate::not();
    not_20.set_input(0, controls[2].clone());
    let mut and_010 = Gate::and();
    set_inputs!(and_010, not_00.get_output(0), not_10.get_output(0));
    let mut and_230 = Gate::and();
    set_inputs!(and_230, not_20.get_output(0), mux_gates[0].get_output(0));
    let mut and_0 = Gate::and();
    set_inputs!(and_0, and_010.get_output(0), and_230.get_output(0));
    let mut gate_0 = make_16_wide_gate_and_set_inputs!(and_0, inputs, 0);
    push!(mux_gates: not_00, not_10, not_20, and_010, and_230, and_0);
    mux_gates.append(&mut gate_0);
    let mut counter = 0;
    c.bench_function("MUX gate of `Gate`s", move |b| b.iter(|| {
        let tmp = inputs[counter % 128].get();
        inputs[counter % 128].set(!tmp);
        let tmp = [Rc::new(Cell::new((counter & 1 == 1).into())),
            Rc::new(Cell::new((counter & 2 == 2).into())),
            Rc::new(Cell::new((counter & 4 == 4).into()))];
        controls[0..3].clone_from_slice(&tmp);
        for gate in &mut mux_gates {
            gate.eval();
        }
        counter += 1;
    }));
}

use std::time::Duration;

criterion_group!{
    name = logic_benches;
    config = Criterion::default().sample_size(1000).measurement_time(Duration::from_secs(30));
    targets = bench_128i_3c_mux, bench_ram_8, bench_128i_3c_mux_gates
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