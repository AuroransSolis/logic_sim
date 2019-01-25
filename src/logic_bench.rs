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

/*fn bench_ram8_of_gates(c: &mut Criterion) {
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
    let mut gates = Vec::new();

}*/

use std::time::Duration;

criterion_group!{
    name = logic_benches;
    config = Criterion::default().sample_size(10_000).measurement_time(Duration::from_secs(60));
    targets = //bench_mux16_8w, bench_mux16_8w_const, bench_ram_8, bench_ram_8_const,
        bench_mux16_8w_gates, bench_mux16_8w_gates_const//, bench_mux16_8w_conditionless,
        //bench_mux16_8w_conditionless_const
}

criterion_main!{logic_benches}