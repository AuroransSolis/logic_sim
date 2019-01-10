#![allow(dead_code, unused_variables)]
#![feature(test)]

use std::rc::Rc;
use std::cell::Cell;

pub mod board;

pub(crate) use board::line::Line;
pub(crate) use board::gate::Gate;
//pub(crate) use board::circuit::Circuit;
//pub(crate) use board::board::Board;

fn main() {
    println!("wewe");
}

extern crate test;

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    use super::{Gate, Line, Rc, Cell};

    #[bench]
    fn bench_128i_3c(b: &mut Bencher) {
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
        b.iter(|| black_box({
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

    #[bench]
    fn bench_ram_8(b: &mut Bencher) {
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
        b.iter(|| black_box({
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
}