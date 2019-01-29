#![allow(dead_code)]

#[macro_use] extern crate criterion;

use criterion::{Criterion, black_box};

pub mod circuit;

use circuit::line::Line;
use circuit::gate::Gate;
use circuit::circuit::Circuit;
use circuit::memory::MSFFRAM8;

struct MUX16_8W {
    inputs: [usize; 128 + 3],
    outputs: [usize; 16]
}

impl MUX16_8W {
    pub(crate) fn new() -> Self {
        MUX16_8W {
            inputs: [0; 128 + 3],
            outputs: [0; 16]
        }
    }
}

impl Gate for MUX16_8W {
    fn get_input(&self, i: usize) -> usize {
        self.inputs[i]
    }

    fn set_input(&mut self, i: usize, new_i: usize) {
        self.inputs[i] = new_i;
    }

    fn num_inputs(&self) -> usize {
        128 + 3
    }

    fn get_output(&self, o: usize) -> usize {
        self.outputs[o]
    }

    fn set_output(&mut self, o: usize, new_o: usize) {
        self.outputs[o] = new_o;
    }

    fn num_outputs(&self) -> usize {
        16
    }

    fn eval(&mut self, lines: &mut Vec<Line>) {
        let which = (0..3).map(|i| (lines[self.inputs[128 + i]].is_high() as usize) << i)
            .sum::<usize>();
        for i in 0..16 {
            let new = lines[self.inputs[which * 16 + i]];
            lines[self.outputs[i]] = new;
        }
    }
}

fn bench_mux16_8w(c: &mut Criterion) {
    let mut circuit = Circuit::new();
    let mux = circuit.add_gate(MUX16_8W::new());
    let mut inputs = Vec::new();
    for _ in 0..128 {
        inputs.push(circuit.add_line(Line::Low));
    }
    let mut controls = Vec::new();
    for _ in 0..3 {
        controls.push(circuit.add_line(Line::Low));
    }
    for i in 0..128 {
        circuit.set_gate_input(mux, i, inputs[i]);
    }
    for i in 128..128 + 3 {
        circuit.set_gate_input(mux, i, controls[i - 128]);
    }
    let mut counter = 0;
    c.bench_function("MUX gate", move |b| b.iter(|| {
        let tmp = circuit.get_line_state(inputs[counter % 128]);
        circuit.set_line(inputs[counter % 128], !tmp);
        for i in 0..3 {
            circuit.set_line(controls[i], (counter >> i & 1 == 1).into());
        };
        circuit.eval();
        counter += 1;
    }));
}

fn bench_mux16_8w_const(c: &mut Criterion) {
    let mut circuit = Circuit::new();
    let mux = circuit.add_gate(MUX16_8W::new());
    let mut inputs = Vec::new();
    for _ in 0..128 {
        inputs.push(circuit.add_line(Line::Low));
    }
    let mut controls = Vec::new();
    for _ in 0..3 {
        controls.push(circuit.add_line(Line::Low));
    }
    for i in 0..128 {
        circuit.set_gate_input(mux, i, inputs[i]);
    }
    for i in 128..128 + 3 {
        circuit.set_gate_input(mux, i, controls[i - 128]);
    }
    c.bench_function("MUX gate (const)", move |b| b.iter(|| black_box(circuit.eval())));
}

fn bench_ram_8(c: &mut Criterion) {
    let mut circuit = Circuit::new();
    let mem = circuit.add_gate(MSFFRAM8::new());
    let mut addr = [0; 8];
    for i in 0..8 {
        addr[i] = circuit.add_line(Line::Low);
        circuit.set_gate_input(mem, i, addr[i]);
    }
    let mut write_val = [0; 8];
    for i in 8..16 {
        write_val[i - 8] = circuit.add_line(Line::Low);
        circuit.set_gate_input(mem, i, write_val[i - 8]);
    }
    let write = circuit.add_line(Line::Low);
    let read = circuit.add_line(Line::Low);
    let clock = circuit.add_line(Line::Low);
    let mut counter = 0;
    c.bench_function("Memory module WHRHCH", move |b| b.iter(|| {
        let tmp = circuit.get_line_state(addr[counter % 8]);
        circuit.set_line(addr[counter % 8], !tmp);
        let tmp = circuit.get_line_state(write_val[7 - (counter % 8)]);
        circuit.set_line(write_val[7 - (counter % 8)], !tmp);
        circuit.set_line(write, Line::High);
        circuit.set_line(clock, Line::High);
        circuit.eval();
        circuit.set_line(write, Line::Low);
        circuit.set_line(clock, Line::Low);
        circuit.set_line(read, Line::High);
        circuit.eval();
        circuit.set_line(read, Line::Low);
        circuit.eval();
        counter += 1;
    }));
}

fn bench_ram_8_const(c: &mut Criterion) {
    let mut circuit = Circuit::new();
    let mem = circuit.add_gate(MSFFRAM8::new());
    let mut addr = [0; 8];
    for i in 0..8 {
        addr[i] = circuit.add_line(Line::Low);
        circuit.set_gate_input(mem, i, addr[i]);
    }
    let mut write_val = [0; 8];
    for i in 8..16 {
        write_val[i - 8] = circuit.add_line(Line::Low);
        circuit.set_gate_input(mem, i, write_val[i - 8]);
    }
    let _write = circuit.add_line(Line::Low);
    let _read = circuit.add_line(Line::Low);
    let _clock = circuit.add_line(Line::Low);
    c.bench_function("Memory module WHRHCH (const)", move |b| b.iter(|| black_box(circuit.eval())));
}

macro_rules! push {
    ($vec:ident: $($gate:ident),+) => {
        $(
            $vec.push($gate);
        )+
    }
}

use circuit::mux::Mux1_2;

fn bench_mux16_8w_gates(c: &mut Criterion) {
    let mut circuit = Circuit::new();
    let mut first_layer_muxes = Vec::new();
    let mut second_layer_muxes = Vec::new();
    let mut third_layer_muxes = Vec::new();
    for _ in 0..16 {
        let m00 = circuit.add_gate(Mux1_2::new());
        let m01 = circuit.add_gate(Mux1_2::new());
        let m02 = circuit.add_gate(Mux1_2::new());
        let m03 = circuit.add_gate(Mux1_2::new());
        let m10 = circuit.add_gate(Mux1_2::new());
        let m11 = circuit.add_gate(Mux1_2::new());
        let m20 = circuit.add_gate(Mux1_2::new());
        circuit.connect_i_single(m10, 0, m00, 0);
        circuit.connect_i_single(m10, 1, m01, 0);
        circuit.connect_i_single(m11, 0, m02, 0);
        circuit.connect_i_single(m11, 1, m03, 0);
        circuit.connect_i_single(m20, 0, m10, 0);
        circuit.connect_i_single(m20, 1, m11, 0);
        push!(first_layer_muxes: m00, m01, m02, m03);
        push!(second_layer_muxes: m10, m11);
        third_layer_muxes.push(m20);
    }
    let mut controls = [0; 3];
    controls[0] = circuit.add_line(Line::Low);
    for mux in &third_layer_muxes {
        circuit.set_gate_input(*mux, 2, controls[0]);
    }
    controls[1] = circuit.add_line(Line::Low);
    for mux in &second_layer_muxes {
        circuit.set_gate_input(*mux, 2, controls[0]);
    }
    controls[2] = circuit.add_line(Line::Low);
    for mux in &first_layer_muxes {
        circuit.set_gate_input(*mux, 2, controls[0]);
    }
    let mut inputs = [0; 128];
    for i in 0..8 {
        for j in 0..16 {
            let new_line = circuit.add_line(Line::Low);
            inputs[i * 16 + j] = new_line;
            circuit.set_gate_input(first_layer_muxes[j * 4 + i / 2], i % 2, new_line);
        }
    }
    let mut counter = 0;
    c.bench_function("MUX gate of `Gate`s", move |b| b.iter(|| {
        let tmp = circuit.get_line_state(inputs[counter % 128]);
        circuit.set_line(inputs[counter % 128], !tmp);
        for i in 0..3 {
            circuit.set_line(controls[i], (counter >> i & 1 == 1).into());
        };
        circuit.eval();
        counter += 1;
    }));
}

fn bench_mux16_8w_gates_const(c: &mut Criterion) {
    let mut circuit = Circuit::new();
    let mut first_layer_muxes = Vec::new();
    let mut second_layer_muxes = Vec::new();
    let mut third_layer_muxes = Vec::new();
    for _ in 0..16 {
        let m00 = circuit.add_gate(Mux1_2::new());
        let m01 = circuit.add_gate(Mux1_2::new());
        let m02 = circuit.add_gate(Mux1_2::new());
        let m03 = circuit.add_gate(Mux1_2::new());
        let m10 = circuit.add_gate(Mux1_2::new());
        let m11 = circuit.add_gate(Mux1_2::new());
        let m20 = circuit.add_gate(Mux1_2::new());
        circuit.connect_i_single(m10, 0, m00, 0);
        circuit.connect_i_single(m10, 1, m01, 0);
        circuit.connect_i_single(m11, 0, m02, 0);
        circuit.connect_i_single(m11, 1, m03, 0);
        circuit.connect_i_single(m20, 0, m10, 0);
        circuit.connect_i_single(m20, 1, m11, 0);
        push!(first_layer_muxes: m00, m01, m02, m03);
        push!(second_layer_muxes: m10, m11);
        third_layer_muxes.push(m20);
    }
    let mut controls = [0; 3];
    controls[0] = circuit.add_line(Line::Low);
    for mux in &third_layer_muxes {
        circuit.set_gate_input(*mux, 2, controls[0]);
    }
    controls[1] = circuit.add_line(Line::Low);
    for mux in &second_layer_muxes {
        circuit.set_gate_input(*mux, 2, controls[0]);
    }
    controls[2] = circuit.add_line(Line::Low);
    for mux in &first_layer_muxes {
        circuit.set_gate_input(*mux, 2, controls[0]);
    }
    let mut inputs = [0; 128];
    for i in 0..8 {
        for j in 0..16 {
            let new_line = circuit.add_line(Line::Low);
            inputs[i * 16 + j] = new_line;
            circuit.set_gate_input(first_layer_muxes[j * 4 + i / 2], i % 2, new_line);
        }
    }
    c.bench_function("MUX gate of `Gate`s (const)", move |b| b.iter(|| {
        black_box(circuit.eval())
    }));
}

struct ConditionlessMux8_16w {
    inputs: [usize; 128 + 3],
    outputs: [usize; 16]
}

impl ConditionlessMux8_16w {
    fn new() -> Self {
        ConditionlessMux8_16w {
            inputs: [0; 128 + 3],
            outputs: [0; 16]
        }
    }
}

impl Gate for ConditionlessMux8_16w {
    fn get_input(&self, i: usize) -> usize {
        self.inputs[i]
    }
    
    fn set_input(&mut self, i: usize, new_i: usize) {
        self.inputs[i] = new_i;
    }
    
    fn num_inputs(&self) -> usize {
        128 + 3
    }
    
    fn get_output(&self, o: usize) -> usize {
        self.outputs[o]
    }
    
    fn set_output(&mut self, o: usize, new_o: usize) {
        self.outputs[o] = new_o;
    }
    
    fn num_outputs(&self) -> usize {
        16
    }
    
    fn eval(&mut self, lines: &mut Vec<Line>) {
        let gates = [
            !lines[self.inputs[128]] * !lines[self.inputs[129]] * !lines[self.inputs[130]],
            !lines[self.inputs[128]] * !lines[self.inputs[129]] *  lines[self.inputs[130]],
            !lines[self.inputs[128]] *  lines[self.inputs[129]] * !lines[self.inputs[130]],
            !lines[self.inputs[128]] *  lines[self.inputs[129]] *  lines[self.inputs[130]],
             lines[self.inputs[128]] * !lines[self.inputs[129]] * !lines[self.inputs[130]],
             lines[self.inputs[128]] * !lines[self.inputs[129]] *  lines[self.inputs[130]],
             lines[self.inputs[128]] *  lines[self.inputs[129]] * !lines[self.inputs[130]],
             lines[self.inputs[128]] *  lines[self.inputs[129]] *  lines[self.inputs[130]]
        ];
        for i in 0..16 {
            for j in 0..8 {
                let n = lines[self.inputs[i + j * 16]] * gates[j] + lines[self.outputs[i]];
                lines[self.outputs[i]] = n;
            }
        }
    }
}

fn bench_mux16_8w_conditionless(c: &mut Criterion) {
    let mut circuit = Circuit::new();
    let mux = circuit.add_gate(ConditionlessMux8_16w::new());
    let mut controls = [0; 3];
    for i in 0..3 {
        controls[i] = circuit.add_line(Line::Low);
        circuit.set_gate_input(mux, 128 + i, controls[i]);
    }
    let mut inputs = [0; 128];
    for i in 0..128 {
        inputs[i] = circuit.add_line(Line::Low);
        circuit.set_gate_input(mux, i, inputs[i]);
    }
    let mut counter = 0;
    c.bench_function("MUX gate without conditionals", move |b| b.iter(|| {
        let tmp = circuit.get_line_state(inputs[counter % 128]);
        circuit.set_line(inputs[counter % 128], !tmp);
        for i in 0..3 {
            circuit.set_line(controls[i], (counter >> i & 1 == 1).into());
        };
        circuit.eval();
        counter += 1;
    }));
}

fn bench_mux16_8w_conditionless_const(c: &mut Criterion) {
    let mut circuit = Circuit::new();
    let mux = circuit.add_gate(ConditionlessMux8_16w::new());
    let mut controls = [0; 3];
    for i in 0..3 {
        controls[i] = circuit.add_line(Line::Low);
        circuit.set_gate_input(mux, 128 + i, controls[i]);
    }
    let mut inputs = [0; 128];
    for i in 0..128 {
        inputs[i] = circuit.add_line(Line::Low);
        circuit.set_gate_input(mux, i, inputs[i]);
    }
    c.bench_function("MUX gate without conditionals (const)", move |b| b.iter(|| {
        black_box(circuit.eval());
    }));
}

fn bench_ram8_of_gates(c: &mut Criterion) {
    let mut circuit = Circuit::new();
    let mut addr = [0; 8];
    for i in 0..8 {
        addr[i] = circuit.add_line(Line::Low);
    }
    let mut write_val = [0; 8];
    for i in 0..8 {
        write_val[i] = circuit.add_line(Line::Low);
    }
    let write = circuit.add_line(Line::Low);
    let read = circuit.add_line(Line::Low);
    let clock = circuit.add_line(Line::Low);
    let _read_lines = make_8bx256_storage(addr, write_val, write, read, clock, &mut circuit);
    let mut counter = 0;
    c.bench_function("Memory module of gates", move |b| b.iter(|| {
        let tmp = circuit.get_line_state(addr[counter % 8]);
        circuit.set_line(addr[counter % 8], !tmp);
        let tmp = circuit.get_line_state(write_val[7 - (counter % 8)]);
        circuit.set_line(write_val[7 - (counter % 8)], !tmp);
        circuit.set_line(write, Line::High);
        circuit.set_line(clock, Line::High);
        circuit.eval();
        circuit.set_line(write, Line::Low);
        circuit.set_line(clock, Line::Low);
        circuit.set_line(read, Line::High);
        circuit.eval();
        circuit.set_line(read, Line::Low);
        circuit.eval();
        counter += 1;
    }));
}

fn bench_ram8_of_gates_const(c: &mut Criterion) {
    let mut circuit = Circuit::new();
    let mut addr = [0; 8];
    for i in 0..8 {
        addr[i] = circuit.add_line(Line::Low);
    }
    let mut write_val = [0; 8];
    for i in 0..8 {
        write_val[i] = circuit.add_line(Line::Low);
    }
    let write = circuit.add_line(Line::Low);
    let read = circuit.add_line(Line::Low);
    let clock = circuit.add_line(Line::Low);
    let _read_lines = make_8bx256_storage(addr, write_val, write, read, clock, &mut circuit);
    c.bench_function("Memory module of gates (const)", move |b| b.iter(|| {
        black_box(circuit.eval());
    }));
}

use std::time::Duration;

criterion_group!{
    name = logic_benches;
    config = Criterion::default().sample_size(10_000).measurement_time(Duration::from_secs(60));
    targets = bench_mux16_8w, bench_mux16_8w_const, bench_ram_8, bench_ram_8_const,
        bench_mux16_8w_gates, bench_mux16_8w_gates_const, bench_mux16_8w_conditionless,
        bench_mux16_8w_conditionless_const, bench_ram8_of_gates, bench_ram8_of_gates_const
}

criterion_main!{logic_benches}

use circuit::simplegate::SimpleGate;
use circuit::basics::Inverter;

// inputs: 0, 1
// output: 7
// gate count: 9
pub fn make_ms_flip_flop(i0: usize, i1: usize, clock: usize, circuit: &mut Circuit) -> usize {
    let m_and1 = circuit.add_gate(SimpleGate::and());
    circuit.set_gate_input(m_and1, 0, i0);
    circuit.set_gate_input(m_and1, 1, clock);
    let m_and2 = circuit.add_gate(SimpleGate::and());
    circuit.set_gate_input(m_and1, 0, i1);
    circuit.set_gate_input(m_and1, 1, clock);
    let m_xor1 = circuit.add_gate(SimpleGate::xor());
    let m_xor2 = circuit.add_gate(SimpleGate::xor());
    circuit.connect_i_single(m_xor1, 0, m_and1, 0);
    circuit.connect_i_single(m_xor1, 1, m_xor2, 0);
    circuit.connect_i_single(m_xor2, 0, m_xor1, 0);
    circuit.connect_i_single(m_xor2, 1, m_and2, 0);
    let not = circuit.add_gate(Inverter::new());
    circuit.set_gate_input(not, 0, clock);
    let s_and1 = circuit.add_gate(SimpleGate::and());
    circuit.set_gate_input(s_and1, 0, i0);
    circuit.set_gate_input(s_and1, 1, clock);
    let s_and2 = circuit.add_gate(SimpleGate::and());
    circuit.set_gate_input(s_and1, 0, i1);
    circuit.set_gate_input(s_and1, 1, clock);
    let s_xor1 = circuit.add_gate(SimpleGate::xor());
    let s_xor2 = circuit.add_gate(SimpleGate::xor());
    circuit.connect_i_single(s_xor1, 0, s_and1, 0);
    circuit.connect_i_single(s_xor1, 1, s_xor2, 0);
    circuit.connect_i_single(s_xor2, 0, s_xor1, 0);
    circuit.connect_i_single(s_xor2, 1, s_and2, 0);
    circuit.get_gate_output(s_xor1, 0)
}

pub fn make_1bx256_storage(input: usize, address: [usize; 8], write: usize, read: usize,
    clock: usize, circuit: &mut Circuit) -> usize {
    let important_dmux_gates = make_1bx256_dmux(address, circuit);
    circuit.set_gate_input(important_dmux_gates[0], 0, input);
    let important_mux_gates = make_1bx256_mux(address, circuit);
    for i in 0..256 {
        let inv = circuit.add_gate(Inverter::new());
        circuit.connect_i_single(inv, 0, important_dmux_gates[i / 2], i % 2);
        let gate_high = circuit.add_gate(SimpleGate::and());
        circuit.set_gate_input(gate_high, 0, clock);
        circuit.connect_i_single(gate_high, 1, important_dmux_gates[i / 2], i % 2);
        let gate_low = circuit.add_gate(SimpleGate::and());
        circuit.set_gate_input(gate_low, 0, clock);
        circuit.connect_i_single(gate_low, 1, inv, 0);
        let write_high = circuit.add_gate(SimpleGate::and());
        circuit.set_gate_input(write_high, 0, write);
        circuit.connect_i_single(write_high, 1, gate_high, 0);
        let write_low = circuit.add_gate(SimpleGate::and());
        circuit.set_gate_input(write_low, 0, write);
        circuit.connect_i_single(write_low, 1, gate_low, 0);
        let bit_i0 = circuit.get_gate_output(write_high, 0);
        let bit_i1 = circuit.get_gate_output(write_low, 0);
        let bit_output = make_ms_flip_flop(bit_i0, bit_i1, clock, circuit);
        let read_gate = circuit.add_gate(SimpleGate::and());
        circuit.set_gate_input(read_gate, 0, bit_output);
        circuit.set_gate_input(read_gate, 1, read);
        circuit.connect_i_single(important_mux_gates[i / 2], i % 2, read_gate, 0);
    }
    important_mux_gates[128]
}

pub fn make_8bx256_storage(address: [usize; 8], write_value: [usize; 8], write: usize, read: usize,
    clock: usize, circuit: &mut Circuit) -> [usize; 8] {
    let mut output_lines = [0; 8];
    for i in 0..8 {
        output_lines[i] = make_1bx256_storage(write_value[i], address, write, read, clock, circuit);
    }
    output_lines
}

macro_rules! dmuxes {
    (($circuit: ident, $controls:ident) $($g0:ident, $g1:ident: $previous_dmux:expr,
        $controls_ind:literal),+) => {
        $(
        let $g0 = $circuit.add_gate(Dmux1_2::new());
        $circuit.connect_i_single($g0, 0, $previous_dmux, 0);
        $circuit.set_gate_input($g0, 1, $controls[$controls_ind]);
        let $g1 = $circuit.add_gate(Dmux1_2::new());
        $circuit.connect_i_single($g1, 0, $previous_dmux, 1);
        $circuit.set_gate_input($g1, 1, $controls[$controls_ind]);
        )+
    }
}

use circuit::mux::Dmux1_2;

pub fn make_1bx256_dmux(controls: [usize; 8], circuit: &mut Circuit) -> [usize; 129] {
    let mut gates = Vec::new();
    let dmux_70 = circuit.add_gate(Dmux1_2::new());
    circuit.set_gate_input(dmux_70, 1, controls[7]);
    gates.push(dmux_70);
    dmuxes! { (circuit, controls)
        dmux_60, dmux_61: dmux_70, 6,

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
    let mut entry_and_outputs = [0; 129];
    for i in (0..128).rev() {
        entry_and_outputs[i] = gates[gates.len() - 1 - i];
    }
    entry_and_outputs[128] = gates[0];
    entry_and_outputs
}

macro_rules! muxes {
    (($circuit:ident, $controls:ident) $($mux:ident: $previous_mux_0:expr, $previous_mux_1:expr,
        $controls_ind:literal),+) => {
        $(
        let $mux = $circuit.add_gate(Mux1_2::new());
        $circuit.connect_i_single($mux, 0, $previous_mux_0, 0);
        $circuit.connect_i_single($mux, 1, $previous_mux_1, 0);
        $circuit.set_gate_input($mux, 2, $controls[$controls_ind]);
        )+
    }
}

pub fn make_1bx256_mux(controls: [usize; 8], circuit: &mut Circuit) -> [usize; 129] {
    let mut gates = Vec::new();
    for _ in 0..128 {
        let mux = circuit.add_gate(Mux1_2::new());
        circuit.set_gate_input(mux, 2, controls[7]);
        gates.push(mux);
    }
    muxes! { (circuit, controls)
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
    let mut inputs_and_output = [0; 129];
    for i in 0..128 {
        inputs_and_output[i] = gates[i];
    }
    inputs_and_output[128] = gates[gates.len() - 1];
    inputs_and_output
}