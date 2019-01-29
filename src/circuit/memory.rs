use circuit::line::{Line, and, or, xor, nand, nor, xnor};
use circuit::circuit::Circuit;

use circuit::gate::Gate;

pub struct NORLatch {
    i0: usize,
    i1: usize,
    clock: usize,
    state: Line,
    output: usize
}

impl NORLatch {
    pub(crate) fn new() -> Self {
        NORLatch {
            i0: 0,
            i1: 0,
            clock: 0,
            state: Line::Low,
            output: 0
        }
    }
}

impl Gate for NORLatch {
    fn get_input(&self, i: usize) -> usize {
        match i {
            0 => self.i0,
            1 => self.i1,
            2 => self.clock,
            _ => panic!("Invalid input.")
        }
    }

    fn set_input(&mut self, i: usize, new_i: usize) {
        match i {
            0 => self.i0 = new_i,
            1 => self.i1 = new_i,
            2 => self.clock = new_i,
            _ => panic!("Attempted to set invalid input.")
        }
    }

    fn num_inputs(&self) -> usize {
        3
    }

    fn get_output(&self, o: usize) -> usize {
        match o {
            0 => self.output,
            _ => panic!("Invalid output.")
        }
    }

    fn set_output(&mut self, o: usize, new_o: usize) {
        match o {
            0 => self.output = new_o,
            _ => panic!("Attempted to set invalid output.")
        }
    }

    fn num_outputs(&self) -> usize {
        1
    }

    fn eval(&mut self, lines: &mut Vec<Line>) {
        if lines[self.clock].is_high() {
            if lines[self.i0].is_high() {
                lines[self.output] = Line::Low;
            }
            if lines[self.i1].is_high() {
                lines[self.output] = Line::High;
            }
        }
    }
}

pub struct MasterSlaveFlipFlop {
    i0: usize,
    i1: usize,
    clock: usize,
    master: Line,
    slave: Line,
    output: usize
}

impl MasterSlaveFlipFlop {
    pub(crate) fn new() -> Self {
        MasterSlaveFlipFlop {
            i0: 0,
            i1: 0,
            clock: 0,
            master: Line::Low,
            slave: Line::Low,
            output: 0
        }
    }
}

impl Gate for MasterSlaveFlipFlop {
    fn get_input(&self, i: usize) -> usize {
        match i {
            0 => self.i0,
            1 => self.i1,
            2 => self.clock,
            _ => panic!("Invalid input.")
        }
    }

    fn set_input(&mut self, i: usize, new_i: usize) {
        match i {
            0 => self.i0 = new_i,
            1 => self.i1 = new_i,
            2 => self.clock = new_i,
            _ => panic!("Attempted to set invalid input.")
        }
    }

    fn num_inputs(&self) -> usize {
        3
    }

    fn get_output(&self, o: usize) -> usize {
        match o {
            0 => self.output,
            _ => panic!("Invalid output.")
        }
    }

    fn set_output(&mut self, o: usize, new_o: usize) {
        match o {
            0 => self.output = new_o,
            _ => panic!("Attempted to set invalid output.")
        }
    }

    fn num_outputs(&self) -> usize {
        1
    }

    fn eval(&mut self, lines: &mut Vec<Line>) {
        if lines[self.clock].is_high() {
            let new_master = xor(lines[self.i0], lines[self.i1]);
            lines[self.output] = new_master;
        } else {
            let new_slave = self.master;
            self.slave = new_slave;
        }
    }
}

/// Spec:
/// 0..8: address
/// 8..16: write value
/// 16: write
/// 17: read
/// 18: clock
pub struct NORLatchRAM8 {
    inputs: [usize; 18],
    storage: [bool; 256 * 8],
    outputs: [usize; 8]
}

impl NORLatchRAM8 {
    fn new() -> Self {
        NORLatchRAM8 {
            inputs: [0; 18],
            storage: [false; 256 * 8],
            outputs: [0; 8]
        }
    }
}

impl Gate for NORLatchRAM8 {
    fn get_input(&self, i: usize) -> usize {
        self.inputs[i]
    }

    fn set_input(&mut self, i: usize, new_i: usize) {
        self.inputs[i] = new_i;
    }

    fn num_inputs(&self) -> usize {
        18
    }

    fn get_output(&self, o: usize) -> usize {
        self.outputs[o]
    }

    fn set_output(&mut self, o: usize, new_o: usize) {
        self.outputs[o] = new_o;
    }

    fn num_outputs(&self) -> usize {
        8
    }

    fn eval(&mut self, lines: &mut Vec<Line>) {
        if lines[self.inputs[18]].is_high() {
            let addr = (0..8).map(|i| (lines[self.inputs[i]].is_high() as usize) << i)
                .sum::<usize>();
            if lines[self.inputs[18]].is_high() {
                if lines[self.inputs[16]].is_high() {
                    for i in 0..8 {
                        let new = lines[self.inputs[8 + i]].into();
                        self.storage[8 * addr + i] = new;
                    }
                }
                if lines[self.inputs[17]].is_high() {
                    for i in 0..8 {
                        lines[self.outputs[i]] = Line::from(self.storage[addr * 8 + i]);
                    }
                }
            }
        }
    }
}

/// Spec:
/// 0..8: address
/// 8..16: write value
/// 16: write
/// 17: read
/// 18: clock
pub struct MSFFRAM8 {
    inputs: [usize; 18],
    storage: [bool; 256 * 8 * 2 + 1],
    outputs: [usize; 8]
}

impl MSFFRAM8 {
    pub(crate) fn new() -> Self {
        MSFFRAM8 {
            inputs: [0; 18],
            storage: [false; 256 * 8 * 2 + 1],
            outputs: [0; 8]
        }
    }
}

impl Gate for MSFFRAM8 {
    fn get_input(&self, i: usize) -> usize {
        self.inputs[i]
    }

    fn set_input(&mut self, i: usize, new_i: usize) {
        self.inputs[i] = new_i;
    }

    fn num_inputs(&self) -> usize {
        18
    }

    fn get_output(&self, o: usize) -> usize {
        self.outputs[o]
    }

    fn set_output(&mut self, o: usize, new_o: usize) {
        self.outputs[o] = new_o;
    }

    fn num_outputs(&self) -> usize {
        8
    }

    fn eval(&mut self, lines: &mut Vec<Line>) {
        if lines[self.inputs[18]].is_high() {
            if lines[self.inputs[16]].is_high() {
                let last_ind = self.storage.len() - 1;
                self.storage[last_ind] = false;
                let addr = (0..8).map(|i| (lines[self.inputs[i]].is_high() as usize) << i)
                    .sum::<usize>();
                for i in 0..8 {
                    let new = lines[self.inputs[8 + i]].into();
                    self.storage[8 * addr + i] = new;
                }
            }
        } else if lines[self.inputs[18]].is_low() && !self.storage[self.storage.len() - 1] {
            let (masters, slaves) = self.storage.split_at_mut(256 * 8);
            slaves.copy_from_slice(&*masters);
            let last = slaves.len() - 1;
            slaves[last] = true;
        }
        if lines[self.inputs[17]].is_high() {
            let addr = (0..8).map(|i| (lines[self.inputs[i]].is_high() as usize) << i)
                .sum::<usize>();
            for i in 0..8 {
                lines[self.outputs[i]] = self.storage[256 * 8 + 8 * addr + i].into();
            }
        }
    }
}

/// Spec:
/// 0..16: address
/// 16..32: write value
/// 32: write
/// 33: read
/// 34: clock
pub struct NORLatchRAM16 {
    inputs: [usize; 35],
    storage: [bool; 65536 * 16],
    outputs: [usize; 16]
}

impl NORLatchRAM16 {
    fn new() -> Self {
        NORLatchRAM16 {
            inputs: [0; 35],
            storage: [false; 65536 * 16],
            outputs: [0; 16]
        }
    }
}

impl Gate for NORLatchRAM16 {
    fn get_input(&self, i: usize) -> usize {
        self.inputs[i]
    }

    fn set_input(&mut self, i: usize, new_i: usize) {
        self.inputs[i] = new_i;
    }

    fn num_inputs(&self) -> usize {
        35
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
        if lines[self.inputs[34]].is_high() {
            let addr = (0..16).map(|i| (lines[self.inputs[i]].is_high() as usize) << i)
                .sum::<usize>();
            if lines[self.inputs[34]].is_high() {
                if lines[self.inputs[32]].is_high() {
                    for i in 0..16 {
                        let new = lines[self.inputs[16 + i]].into();
                        self.storage[16 * addr + i] = new;
                    }
                }
                if lines[self.inputs[33]].is_high() {
                    for i in 0..16 {
                        lines[self.outputs[i]] = Line::from(self.storage[addr * 16 + i]);
                    }
                }
            }
        }
    }
}

/// Spec:
/// 0..16: address
/// 16..32: write value
/// 32: write
/// 33: read
/// 34: clock
pub struct MSFFRAM16 {
    inputs: [usize; 35],
    storage: [bool; 65536 * 16 * 2 + 1],
    outputs: [usize; 16]
}

impl MSFFRAM16 {
    pub(crate) fn new() -> Self {
        MSFFRAM16 {
            inputs: [0; 35],
            storage: [false; 65536 * 16 * 2 + 1],
            outputs: [0; 16]
        }
    }
}

impl Gate for MSFFRAM16 {
    fn get_input(&self, i: usize) -> usize {
        self.inputs[i]
    }

    fn set_input(&mut self, i: usize, new_i: usize) {
        self.inputs[i] = new_i;
    }

    fn num_inputs(&self) -> usize {
        35
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
        if lines[self.inputs[34]].is_high() {
            if lines[self.inputs[32]].is_high() {
                let last_ind = self.storage.len() - 1;
                self.storage[last_ind] = false;
                let addr = (0..16).map(|i| (lines[self.inputs[i]].is_high() as usize) << i)
                    .sum::<usize>();
                for i in 0..16 {
                    let new = lines[self.inputs[16 + i]].into();
                    self.storage[16 * addr + i] = new;
                }
            }
        } else if lines[self.inputs[34]].is_low() && !self.storage[self.storage.len() - 1] {
            let (masters, slaves) = self.storage.split_at_mut(65536 * 16);
            slaves.copy_from_slice(&*masters);
            let last = slaves.len() - 1;
            slaves[last] = true;
        }
        if lines[self.inputs[33]].is_high() {
            let addr = (0..16).map(|i| (lines[self.inputs[i]].is_high() as usize) << i)
                .sum::<usize>();
            for i in 0..16 {
                lines[self.outputs[i]] = self.storage[65536 * 16 + 16 * addr + i].into();
            }
        }
    }
}