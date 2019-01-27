#[macro_use] extern crate criterion;

use criterion::{Criterion, black_box};

pub mod board;

use board::line::Line;
use board::gate::Gate;

use std::cell::Cell;
use std::rc::Rc;

fn bench_mux16_8w(c: &mut Criterion) {
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
    let mut inputs = Vec::new();
    for _ in 0..128 {
        inputs.push(Rc::new(Cell::new(Line::Low)));
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

fn bench_mux16_8w_const(c: &mut Criterion) {
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
    let mut inputs = Vec::new();
    for _ in 0..128 {
        inputs.push(Rc::new(Cell::new(Line::Low)));
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

fn bench_mux16_8w_gates(c: &mut Criterion) {
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

fn bench_mux16_8w_gates_const(c: &mut Criterion) {
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
    c.bench_function("MUX gate of `Gate`s (const)", move |b| b.iter(|| {
        for gate in &mut mux_gates {
            black_box(gate.eval());
        }
    }));
}

fn bench_mux16_8w_conditionless(c: &mut Criterion) {
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
    let mut inputs = Vec::new();
    for _ in 0..128 {
        inputs.push(Rc::new(Cell::new(Line::Low)));
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

fn bench_mux16_8w_conditionless_const(c: &mut Criterion) {
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
    let mut inputs = Vec::new();
    for _ in 0..128 {
        inputs.push(Rc::new(Cell::new(Line::Low)));
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

fn bench_ram8_of_gates(c: &mut Criterion) {
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
    let mut gates = make_8bx256_storage(addr.clone(), write_val.clone(), write.clone(),
        read.clone(), clock.clone());
    let mut counter = 0;
    c.bench_function("Memory module (gates: 28664)", move |b| b.iter(|| {
        let tmp = addr[counter % 8].get();
        addr[counter % 8].set(!tmp);
        let tmp = write_val[7 - (counter % 8)].get();
        write_val[7 - (counter % 8)].set(!tmp);
        write.set(Line::High);
        for gate in &mut gates {
            gate.eval();
        }
        write.set(Line::Low);
        read.set(Line::High);
        for gate in &mut gates {
            gate.eval();
        }
        read.set(Line::Low);
        for gate in &mut gates {
            gate.eval();
        }
        counter += 1;
    }));
}

use std::time::Duration;

criterion_group!{
    name = logic_benches;
    config = Criterion::default().sample_size(10_000).measurement_time(Duration::from_secs(60));
    targets = bench_mux16_8w, bench_mux16_8w_const, bench_ram_8, bench_ram_8_const,
        bench_mux16_8w_gates, bench_mux16_8w_gates_const, bench_mux16_8w_conditionless,
        bench_mux16_8w_conditionless_const, bench_ram8_of_gates
}

criterion_main!{logic_benches}

// inputs: 0, 1
// output: 7
// gate count: 9
fn make_ms_flip_flop(i0: Rc<Cell<Line>>, i1: Rc<Cell<Line>>, clock: Rc<Cell<Line>>) -> Vec<Gate> {
    let mut gates = Vec::new();
    let mut m_and1 = Gate::and();
    unsafe { m_and1.set_inputs(&[i0, clock.clone()]) };
    let mut m_and2 = Gate::and();
    unsafe { m_and2.set_inputs(&[i1, clock.clone()]) };
    let mut m_xor1 = Gate::xor();
    let mut m_xor2 = Gate::xor();
    unsafe { m_xor1.set_inputs(&[m_and1.get_output(0), m_xor2.get_output(0)]) };
    unsafe { m_xor2.set_inputs(&[m_xor1.get_output(0), m_and2.get_output(0)]) };
    let mut not = Gate::not();
    not.set_input(0, clock);
    let mut s_and1 = Gate::and();
    unsafe { s_and1.set_inputs(&[m_xor2.get_output(0), not.get_output(0)]) };
    let mut s_and2 = Gate::and();
    unsafe { s_and2.set_inputs(&[m_xor1.get_output(0), not.get_output(0)]) };
    let mut s_xor1 = Gate::xor();
    let mut s_xor2 = Gate::xor();
    unsafe { s_xor1.set_inputs(&[s_and1.get_output(0), s_xor2.get_output(0)]) };
    unsafe { s_xor2.set_inputs(&[s_xor1.get_output(0), s_and2.get_output(0)]) };
    push!{gates: m_and1, m_and2, m_xor1, m_xor2, not, s_and1, s_and2, s_xor1, s_xor2};
    gates
}

fn make_1bx256_storage(input: Rc<Cell<Line>>, address: [Rc<Cell<Line>>; 8], write: Rc<Cell<Line>>,
    read: Rc<Cell<Line>>, clock: Rc<Cell<Line>>) -> Vec<Gate> {
    let mut dmux_gates = make_1bx256_dmux(address.clone());
    dmux_gates[0].set_input(0, input);
    let mut mux_gates = make_1bx256_mux(address.clone());
    mux_gates.push(Gate::not());
    let mut bit_setters = Vec::new();
    let mut bit_readers = Vec::new();
    let mut bits = Vec::new();
    for i in 0..256 {
        let mut inv = Gate::not();
        inv.set_input(0, dmux_gates[i / 2].get_output(0));
        let mut gate_high = Gate::and();
        gate_high.set_input(0, clock.clone());
        gate_high.set_input(1, dmux_gates[i / 2].get_output(0));
        let mut gate_low = Gate::and();
        gate_low.set_input(0, clock.clone());
        gate_low.set_input(1, inv.get_output(0));
        let mut write_high = Gate::and();
        write_high.set_input(0, write.clone());
        write_high.set_input(1, gate_high.get_output(0));
        let mut write_low = Gate::and();
        write_low.set_input(0, write.clone());
        write_low.set_input(1, gate_low.get_output(0));
        let mut bit = make_ms_flip_flop(write_high.get_output(0), write_low.get_output(0),
            clock.clone());
        push!{bit_setters: inv, gate_high, gate_low};
        let mut read_gate = Gate::and();
        read_gate.set_input(0, bit[7].get_output(0));
        read_gate.set_input(1, read.clone());
        mux_gates[i / 2].set_input(i % 2, read_gate.get_output(0));
        bit_readers.push(read_gate);
        bits.append(&mut bit);
    }
    let mut gates = dmux_gates;
    gates.append(&mut bit_setters);
    gates.append(&mut bits);
    gates.append(&mut bit_readers);
    bits.append(&mut mux_gates);
    gates
}

pub fn make_8bx256_storage(address: [Rc<Cell<Line>>; 8], write_value: [Rc<Cell<Line>>; 8],
    write: Rc<Cell<Line>>, read: Rc<Cell<Line>>, clock: Rc<Cell<Line>>) -> Vec<Gate> {
    let mut gates = Vec::new();
    for i in 0..8 {
        gates.append(&mut make_1bx256_storage(write_value[i].clone(), address.clone(), write.clone(),
            read.clone(), clock.clone()));
    }
    gates
}

macro_rules! dmuxes {
    (($controls:ident) $($g0:ident, $g1:ident: $previous_dmux:expr, $controls_ind:literal),+) => {
        $(
        let mut $g0 = Gate::dmux_1b_2w();
        $g0.set_input(0, $previous_dmux.get_output(0));
        $g0.set_input(1, $controls[$controls_ind].clone());
        let mut $g1 = Gate::dmux_1b_2w();
        $g1.set_input(0, $controls[$controls_ind].clone());
        $g1.set_input(1, $previous_dmux.get_output(0));
        )+
    }
}

fn make_1bx256_dmux(controls: [Rc<Cell<Line>>; 8]) -> Vec<Gate> {
    let mut gates = Vec::new();
    let mut dmux_70 = Gate::dmux_1b_2w();
    dmux_70.set_input(1, controls[7].clone());
    gates.push(dmux_70);
    dmuxes! { (controls)
        dmux_60, dmux_61: gates[0], 6,
        dmux_61, dmux_62: gates[0], 6,

        dmux_50, dmux_51: dmux_60, 5,
        dmux_52, dmux_53: dmux_61, 5,

        dmux_40, dmux_41: dmux_50, 4,
        dmux_42, dmux_43: dmux_51, 4,
        dmux_44, dmux_45: dmux_52, 4,
        dmux_46, dmux_47: dmux_53, 4,

        dmux_30, dmux_31: dmux_40, 3,
        dmux_32, dmux_33: dmux_41, 3,
        dmux_34, dmux_35: dmux_42, 3,
        dmux_36, dmux_37: dmux_43, 3,
        dmux_38, dmux_39: dmux_44, 3,
        dmux_310, dmux_311: dmux_45, 3,
        dmux_312, dmux_313: dmux_46, 3,
        dmux_314, dmux_315: dmux_47, 3,

        dmux_20, dmux_21: dmux_30, 2,
        dmux_22, dmux_23: dmux_31, 2,
        dmux_24, dmux_25: dmux_32, 2,
        dmux_26, dmux_27: dmux_33, 2,
        dmux_28, dmux_29: dmux_34, 2,
        dmux_210, dmux_211: dmux_35, 2,
        dmux_212, dmux_213: dmux_36, 2,
        dmux_214, dmux_215: dmux_37, 2,
        dmux_216, dmux_217: dmux_38, 2,
        dmux_218, dmux_219: dmux_39, 2,
        dmux_220, dmux_221: dmux_310, 2,
        dmux_222, dmux_223: dmux_311, 2,
        dmux_224, dmux_225: dmux_312, 2,
        dmux_226, dmux_227: dmux_313, 2,
        dmux_228, dmux_229: dmux_314, 2,
        dmux_230, dmux_231: dmux_315, 2,

        dmux_10, dmux_11: dmux_20, 1,
        dmux_12, dmux_13: dmux_21, 1,
        dmux_14, dmux_15: dmux_22, 1,
        dmux_16, dmux_17: dmux_23, 1,
        dmux_18, dmux_19: dmux_24, 1,
        dmux_110, dmux_111: dmux_25, 1,
        dmux_112, dmux_113: dmux_26, 1,
        dmux_114, dmux_115: dmux_27, 1,
        dmux_116, dmux_117: dmux_28, 1,
        dmux_118, dmux_119: dmux_29, 1,
        dmux_120, dmux_121: dmux_210, 1,
        dmux_122, dmux_123: dmux_211, 1,
        dmux_124, dmux_125: dmux_212, 1,
        dmux_126, dmux_127: dmux_213, 1,
        dmux_128, dmux_129: dmux_214, 1,
        dmux_130, dmux_131: dmux_215, 1,
        dmux_132, dmux_133: dmux_216, 1,
        dmux_134, dmux_135: dmux_217, 1,
        dmux_136, dmux_137: dmux_218, 1,
        dmux_138, dmux_139: dmux_219, 1,
        dmux_140, dmux_141: dmux_220, 1,
        dmux_142, dmux_143: dmux_221, 1,
        dmux_144, dmux_145: dmux_222, 1,
        dmux_146, dmux_147: dmux_223, 1,
        dmux_148, dmux_149: dmux_224, 1,
        dmux_150, dmux_151: dmux_225, 1,
        dmux_152, dmux_153: dmux_226, 1,
        dmux_154, dmux_155: dmux_227, 1,
        dmux_156, dmux_157: dmux_228, 1,
        dmux_158, dmux_159: dmux_229, 1,
        dmux_160, dmux_161: dmux_230, 1,
        dmux_162, dmux_163: dmux_231, 1,

        dmux_00, dmux_01: dmux_10, 0,
        dmux_02, dmux_03: dmux_11, 0,
        dmux_04, dmux_05: dmux_12, 0,
        dmux_06, dmux_07: dmux_13, 0,
        dmux_08, dmux_09: dmux_14, 0,
        dmux_010, dmux_011: dmux_15, 0,
        dmux_012, dmux_013: dmux_16, 0,
        dmux_014, dmux_015: dmux_17, 0,
        dmux_016, dmux_017: dmux_18, 0,
        dmux_018, dmux_019: dmux_19, 0,
        dmux_020, dmux_021: dmux_110, 0,
        dmux_022, dmux_023: dmux_111, 0,
        dmux_024, dmux_025: dmux_112, 0,
        dmux_026, dmux_027: dmux_113, 0,
        dmux_028, dmux_029: dmux_114, 0,
        dmux_030, dmux_031: dmux_115, 0,
        dmux_032, dmux_033: dmux_116, 0,
        dmux_034, dmux_035: dmux_117, 0,
        dmux_036, dmux_037: dmux_118, 0,
        dmux_038, dmux_039: dmux_119, 0,
        dmux_040, dmux_041: dmux_120, 0,
        dmux_042, dmux_043: dmux_121, 0,
        dmux_044, dmux_045: dmux_122, 0,
        dmux_046, dmux_047: dmux_123, 0,
        dmux_048, dmux_049: dmux_124, 0,
        dmux_050, dmux_051: dmux_125, 0,
        dmux_052, dmux_053: dmux_126, 0,
        dmux_054, dmux_055: dmux_127, 0,
        dmux_056, dmux_057: dmux_128, 0,
        dmux_058, dmux_059: dmux_129, 0,
        dmux_060, dmux_061: dmux_130, 0,
        dmux_062, dmux_063: dmux_131, 0,
        dmux_064, dmux_065: dmux_132, 0,
        dmux_066, dmux_067: dmux_133, 0,
        dmux_068, dmux_069: dmux_134, 0,
        dmux_070, dmux_071: dmux_135, 0,
        dmux_072, dmux_073: dmux_136, 0,
        dmux_074, dmux_075: dmux_137, 0,
        dmux_076, dmux_077: dmux_138, 0,
        dmux_078, dmux_079: dmux_139, 0,
        dmux_080, dmux_081: dmux_140, 0,
        dmux_082, dmux_083: dmux_141, 0,
        dmux_084, dmux_085: dmux_142, 0,
        dmux_086, dmux_087: dmux_143, 0,
        dmux_088, dmux_089: dmux_144, 0,
        dmux_090, dmux_091: dmux_145, 0,
        dmux_092, dmux_093: dmux_146, 0,
        dmux_094, dmux_095: dmux_147, 0,
        dmux_096, dmux_097: dmux_148, 0,
        dmux_098, dmux_099: dmux_149, 0,
        dmux_0100, dmux_0101: dmux_150, 0,
        dmux_0102, dmux_0103: dmux_151, 0,
        dmux_0104, dmux_0105: dmux_152, 0,
        dmux_0106, dmux_0107: dmux_153, 0,
        dmux_0108, dmux_0109: dmux_154, 0,
        dmux_0110, dmux_0111: dmux_155, 0,
        dmux_0112, dmux_0113: dmux_156, 0,
        dmux_0114, dmux_0115: dmux_157, 0,
        dmux_0116, dmux_0117: dmux_158, 0,
        dmux_0118, dmux_0119: dmux_159, 0,
        dmux_0120, dmux_0121: dmux_160, 0,
        dmux_0122, dmux_0123: dmux_161, 0,
        dmux_0124, dmux_0125: dmux_162, 0,
        dmux_0126, dmux_0127: dmux_163, 0
    };
    push!(gates:
        dmux_60, dmux_61,
        dmux_50, dmux_51, dmux_52, dmux_53,
        dmux_40, dmux_41, dmux_42, dmux_43, dmux_44, dmux_45, dmux_46, dmux_47,
        dmux_30, dmux_31, dmux_32, dmux_33, dmux_34, dmux_35, dmux_36, dmux_37, dmux_38, dmux_39,
        dmux_310, dmux_311, dmux_312, dmux_313, dmux_314, dmux_315,
        dmux_20, dmux_21, dmux_22, dmux_23, dmux_24, dmux_25, dmux_26, dmux_27, dmux_28, dmux_29,
        dmux_210, dmux_211, dmux_212, dmux_213, dmux_214, dmux_215, dmux_216, dmux_217, dmux_218,
        dmux_219, dmux_220, dmux_221, dmux_222, dmux_223, dmux_224, dmux_225, dmux_226, dmux_227,
        dmux_228, dmux_229, dmux_230, dmux_231,
        dmux_10, dmux_11, dmux_12, dmux_13, dmux_14, dmux_15, dmux_16, dmux_17, dmux_18, dmux_19,
        dmux_110, dmux_111, dmux_112, dmux_113, dmux_114, dmux_115, dmux_116, dmux_117, dmux_118,
        dmux_119, dmux_120, dmux_121, dmux_122, dmux_123, dmux_124, dmux_125, dmux_126, dmux_127,
        dmux_128, dmux_129, dmux_130, dmux_131, dmux_132, dmux_133, dmux_134, dmux_135, dmux_136,
        dmux_137, dmux_138, dmux_139, dmux_140, dmux_141, dmux_142, dmux_143, dmux_144, dmux_145,
        dmux_146, dmux_147, dmux_148, dmux_149, dmux_150, dmux_151, dmux_152, dmux_153, dmux_154,
        dmux_155, dmux_156, dmux_157, dmux_158, dmux_159, dmux_160, dmux_161, dmux_162, dmux_163,
        dmux_00, dmux_01, dmux_02, dmux_03, dmux_04, dmux_05, dmux_06, dmux_07, dmux_08, dmux_09,
        dmux_010, dmux_011, dmux_012, dmux_013, dmux_014, dmux_015, dmux_016, dmux_017, dmux_018,
        dmux_019, dmux_020, dmux_021, dmux_022, dmux_023, dmux_024, dmux_025, dmux_026, dmux_027,
        dmux_028, dmux_029, dmux_030, dmux_031, dmux_032, dmux_033, dmux_034, dmux_035, dmux_036,
        dmux_037, dmux_038, dmux_039, dmux_040, dmux_041, dmux_042, dmux_043, dmux_044, dmux_045,
        dmux_046, dmux_047, dmux_048, dmux_049, dmux_050, dmux_051, dmux_052, dmux_053, dmux_054,
        dmux_055, dmux_056, dmux_057, dmux_058, dmux_059, dmux_060, dmux_061, dmux_062, dmux_063,
        dmux_064, dmux_065, dmux_066, dmux_067, dmux_068, dmux_069, dmux_070, dmux_071, dmux_072,
        dmux_073, dmux_074, dmux_075, dmux_076, dmux_077, dmux_078, dmux_079, dmux_080, dmux_081,
        dmux_082, dmux_083, dmux_084, dmux_085, dmux_086, dmux_087, dmux_088, dmux_089, dmux_090,
        dmux_091, dmux_092, dmux_093, dmux_094, dmux_095, dmux_096, dmux_097, dmux_098, dmux_099,
        dmux_0100, dmux_0101, dmux_0102, dmux_0103, dmux_0104, dmux_0105, dmux_0106, dmux_0107,
        dmux_0108, dmux_0109, dmux_0110, dmux_0111, dmux_0112, dmux_0113, dmux_0114, dmux_0115,
        dmux_0116, dmux_0117, dmux_0118, dmux_0119, dmux_0120, dmux_0121, dmux_0122, dmux_0123,
        dmux_0124, dmux_0125, dmux_0126, dmux_0127
    );
    gates
}

macro_rules! muxes {
    (($controls:ident) $($m:ident: $pm_0:expr, $pm_1:expr,
        $controls_ind:literal),+) => {
        $(
        let mut $m = Gate::mux_1b_2w();
        $m.set_input(0, $pm_0.get_output(0));
        $m.set_input(1, $pm_1.get_output(0));
        $m.set_input(2, $controls[$controls_ind].clone());
        )+
    }
}

fn make_1bx256_mux(address: [Rc<Cell<Line>>; 8]) -> Vec<Gate> {
    let mut gates = Vec::new();
    for _ in 0..128 {
        let mut mux = Gate::mux_1b_2w();
        mux.set_input(2, address[7].clone());
        gates.push(mux);
    }
    muxes! { (address)
        mux_60: gates[0], gates[1], 6,
        mux_61: gates[2], gates[3], 6,
        mux_62: gates[4], gates[5], 6,
        mux_63: gates[6], gates[7], 6,
        mux_64: gates[8], gates[9], 6,
        mux_65: gates[10], gates[11], 6,
        mux_66: gates[12], gates[13], 6,
        mux_67: gates[14], gates[15], 6,
        mux_68: gates[16], gates[17], 6,
        mux_69: gates[18], gates[19], 6,
        mux_610: gates[20], gates[21], 6,
        mux_611: gates[22], gates[23], 6,
        mux_612: gates[24], gates[25], 6,
        mux_613: gates[26], gates[27], 6,
        mux_614: gates[28], gates[29], 6,
        mux_615: gates[30], gates[31], 6,
        mux_616: gates[32], gates[33], 6,
        mux_617: gates[34], gates[35], 6,
        mux_618: gates[36], gates[37], 6,
        mux_619: gates[38], gates[39], 6,
        mux_620: gates[40], gates[41], 6,
        mux_621: gates[42], gates[43], 6,
        mux_622: gates[44], gates[45], 6,
        mux_623: gates[46], gates[47], 6,
        mux_624: gates[48], gates[49], 6,
        mux_625: gates[50], gates[51], 6,
        mux_626: gates[52], gates[53], 6,
        mux_627: gates[54], gates[55], 6,
        mux_628: gates[56], gates[57], 6,
        mux_629: gates[58], gates[59], 6,
        mux_630: gates[60], gates[61], 6,
        mux_631: gates[62], gates[63], 6,
        mux_632: gates[64], gates[65], 6,
        mux_633: gates[66], gates[67], 6,
        mux_634: gates[68], gates[69], 6,
        mux_635: gates[70], gates[71], 6,
        mux_636: gates[72], gates[73], 6,
        mux_637: gates[74], gates[75], 6,
        mux_638: gates[76], gates[77], 6,
        mux_639: gates[78], gates[79], 6,
        mux_640: gates[80], gates[81], 6,
        mux_641: gates[82], gates[83], 6,
        mux_642: gates[84], gates[85], 6,
        mux_643: gates[86], gates[87], 6,
        mux_644: gates[88], gates[89], 6,
        mux_645: gates[90], gates[91], 6,
        mux_646: gates[92], gates[93], 6,
        mux_647: gates[94], gates[95], 6,
        mux_648: gates[96], gates[97], 6,
        mux_649: gates[98], gates[99], 6,
        mux_650: gates[100], gates[101], 6,
        mux_651: gates[102], gates[103], 6,
        mux_652: gates[104], gates[105], 6,
        mux_653: gates[106], gates[107], 6,
        mux_654: gates[108], gates[109], 6,
        mux_655: gates[110], gates[111], 6,
        mux_656: gates[112], gates[113], 6,
        mux_657: gates[114], gates[115], 6,
        mux_658: gates[116], gates[117], 6,
        mux_659: gates[118], gates[119], 6,
        mux_660: gates[120], gates[121], 6,
        mux_661: gates[122], gates[123], 6,
        mux_662: gates[124], gates[125], 6,
        mux_663: gates[126], gates[127], 6,

        mux_50: mux_60, mux_61, 5,
        mux_51: mux_62, mux_63, 5,
        mux_52: mux_64, mux_65, 5,
        mux_53: mux_66, mux_67, 5,
        mux_54: mux_68, mux_69, 5,
        mux_55: mux_610, mux_611, 5,
        mux_56: mux_612, mux_613, 5,
        mux_57: mux_614, mux_615, 5,
        mux_58: mux_616, mux_617, 5,
        mux_59: mux_618, mux_619, 5,
        mux_510: mux_620, mux_621, 5,
        mux_511: mux_622, mux_623, 5,
        mux_512: mux_624, mux_625, 5,
        mux_513: mux_626, mux_627, 5,
        mux_514: mux_628, mux_629, 5,
        mux_515: mux_630, mux_631, 5,
        mux_516: mux_632, mux_633, 5,
        mux_517: mux_634, mux_635, 5,
        mux_518: mux_636, mux_637, 5,
        mux_519: mux_638, mux_639, 5,
        mux_520: mux_640, mux_641, 5,
        mux_521: mux_642, mux_643, 5,
        mux_522: mux_644, mux_645, 5,
        mux_523: mux_646, mux_647, 5,
        mux_524: mux_648, mux_649, 5,
        mux_525: mux_650, mux_651, 5,
        mux_526: mux_652, mux_653, 5,
        mux_527: mux_654, mux_655, 5,
        mux_528: mux_656, mux_657, 5,
        mux_529: mux_658, mux_659, 5,
        mux_530: mux_660, mux_661, 5,
        mux_531: mux_662, mux_663, 5,

        mux_40: mux_50, mux_51, 4,
        mux_41: mux_52, mux_53, 4,
        mux_42: mux_54, mux_55, 4,
        mux_43: mux_56, mux_57, 4,
        mux_44: mux_58, mux_59, 4,
        mux_45: mux_510, mux_511, 4,
        mux_46: mux_512, mux_513, 4,
        mux_47: mux_514, mux_515, 4,
        mux_48: mux_516, mux_517, 4,
        mux_49: mux_518, mux_519, 4,
        mux_410: mux_520, mux_521, 4,
        mux_411: mux_522, mux_523, 4,
        mux_412: mux_524, mux_525, 4,
        mux_413: mux_526, mux_527, 4,
        mux_414: mux_528, mux_529, 4,
        mux_415: mux_530, mux_531, 4,

        mux_30: mux_40, mux_41, 3,
        mux_31: mux_42, mux_43, 3,
        mux_32: mux_44, mux_45, 3,
        mux_33: mux_46, mux_47, 3,
        mux_34: mux_48, mux_49, 3,
        mux_35: mux_410, mux_411, 3,
        mux_36: mux_412, mux_413, 3,
        mux_37: mux_414, mux_415, 3,

        mux_20: mux_30, mux_31, 2,
        mux_21: mux_32, mux_33, 2,
        mux_22: mux_34, mux_35, 2,
        mux_23: mux_36, mux_37, 2,

        mux_10: mux_20, mux_21, 1,
        mux_11: mux_22, mux_23, 1,

        mux_00: mux_10, mux_11, 0
    };
    push!(gates:
        mux_00,
        mux_10, mux_11,
        mux_20, mux_21, mux_22, mux_23,
        mux_30, mux_31, mux_32, mux_33, mux_34, mux_35, mux_36, mux_37,
        mux_40, mux_41, mux_42, mux_43, mux_44, mux_45, mux_46, mux_47, mux_48, mux_49, mux_410,
        mux_411, mux_412, mux_413, mux_414, mux_415,
        mux_50, mux_51, mux_52, mux_53, mux_54, mux_55, mux_56, mux_57, mux_58, mux_59, mux_510,
        mux_511, mux_512, mux_513, mux_514, mux_515, mux_516, mux_517, mux_518, mux_519, mux_520,
        mux_521, mux_522, mux_523, mux_524, mux_525, mux_526, mux_527, mux_528, mux_529, mux_530,
        mux_531,
        mux_60, mux_61, mux_62, mux_63, mux_64, mux_65, mux_66, mux_67, mux_68, mux_69, mux_610,
        mux_611, mux_612, mux_613, mux_614, mux_615, mux_616, mux_617, mux_618, mux_619, mux_620,
        mux_621, mux_622, mux_623, mux_624, mux_625, mux_626, mux_627, mux_628, mux_629, mux_630,
        mux_631, mux_632, mux_633, mux_634, mux_635, mux_636, mux_637, mux_638, mux_639, mux_640,
        mux_641, mux_642, mux_643, mux_644, mux_645, mux_646, mux_647, mux_648, mux_649, mux_650,
        mux_651, mux_652, mux_653, mux_654, mux_655, mux_656, mux_657, mux_658, mux_659, mux_660,
        mux_661, mux_662, mux_663
    );
    gates
}