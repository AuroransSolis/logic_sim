use board::line::{Line, HIGH, LOW, DISCONNECTED, not, and, or, xor, nand, nor, xnor};
use board::circuit::Circuit;

use board::gate::Gate;

struct MasterSlaveFlipFlop {
    i0: usize,
    i1: usize,
    clock: usize,
    master: Line,
    slave: Line,
    output: usize
}

impl MasterSlaveFlipFlop {
    pub(crate) fn new(i0: usize, i1: usize, clock: usize, master_init: Line, slave_init: Line,
        output: usize) -> Self {
        MasterSlaveFlipFlop {
            i0,
            i1,
            clock,
            master: master_init,
            slave: slave_init,
            output
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

    fn eval(&mut self, circuit: &mut Circuit<Gate>) {
        let current_output = circuit.lines[self.clock];
        if current_output.is_high() {
            let new_master = xor(circuit.lines[self.i0], circuit.lines[self.i1]);
            circuit.lines[self.output] = new_master;
        } else {
            let new_slave = self.master;
            self.slave = new_slave;
        }
    }
}

impl Gate {
    /// Spec:
    /// 0: set latch low
    /// 1: set latch high
    pub fn nor_latch() -> Self {
        Gate::new_ns(2, 1, |inputs, output| {
            if xor(inputs[0].get(), inputs[1].get()) {
                if inputs[0] {
                    output[0].set(LOW);
                } else if inputs[1].get() {
                    output[0].set(HIGH);
                }
            }
        })
    }

    /// Spec:
    /// 0..8: address
    /// 8..16: write value
    /// 16: write
    /// 17: read
    /// 18: clock
    pub fn master_slave_flip_flop_ram_8() -> Self {
        // Storage:
        // 1 byte as smallest addressable section of memory => 8
        // 256 bytes to fill the available address space => 256
        // Master/slave flip-flop for stable reads => 2
        // 256 * 8 * 2 `Line`s
        // Operation: when the clock is high, writes are done to the first half of the memory slice,
        // and when the clock drops low the data is copied to the second half. The last `Line` is
        // used as a flag to tell whether or not the data write has happened.
        Gate::new_s(19, 256 * 8 * 2 + 1, 8, |inputs, storage, outputs| {
            if inputs[18].get() {
                if inputs[16].get() {
                    let last_ind = storage.len() - 1;
                    storage[last_ind] = LOW;
                    let addr = (0..8).map(|i| (inputs[i].get() as usize) << i).sum::<_>();
                    for i in 0..8 {
                        // The `| Line::Low` below is to ensure that storage is always
                        // `Line::High` or `Line::Low`, since
                        // `Line::Disconnected | Line::Low == Line::Low`.
                        storage[8 * addr + i] = inputs[8 + i].get();
                    }
                }
            } else if nor(inputs[18].get(), storage[storage.len() - 1]) {
                let (masters, slaves) = storage.split_at_mut(256 * 8);
                slaves.copy_from_slice(masters);
                let last = slaves.len() - 1;
                slaves[last] = HIGH;
            }
            if and(inputs[18].get(), inputs[17].get()) {
                let addr = (0..8).map(|i| (inputs[i].get() as usize) << i).sum::<_>();
                for i in 0..8 {
                    outputs[i].set(storage[256 * 8 + 8 * addr + i]);
                }
            }
        })
    }

    /// Spec:
    /// 0..16: address
    /// 16..32: write value
    /// 32: write
    /// 33: read
    /// 34: clock
    pub fn master_slave_flip_flop_ram_16() -> Self {
        // Storage:
        // 2 bytes as smallest addressable section of memory => 16
        // 65536 bytes to fill the available address space => 65536
        // Master/slave flip-flop for stable reads => 2
        // 16 * 65536 * 2 `Line`s
        // Operation: when the clock is high, writes are done to the first half of the memory slice,
        // and when the clock drops low the data is copied to the second half. The last `Line` is
        // used as a flag to tell whether or not the data write has happened.
        Gate::new_s(35, 16 * 65536 * 2 + 1, 16, |inputs, storage, outputs| {
            if inputs[34].get() {
                if inputs[32].get() {
                    let last_ind = storage.len() - 1;
                    storage[last_ind] = LOW;
                    let addr = (0..16).map(|i| (inputs[i].get() as usize) << i).sum::<_>();
                    for i in 0..16 {
                        storage[16 * addr + i] = inputs[16 + i].get();
                    }
                }
            } else if and(!inputs[34].get(), inputs[32].get()) {
                let (masters, slaves) = storage.split_at_mut(16 * 65536);
                slaves.copy_from_slice(&*masters);
                let last = slaves.len() - 1;
                slaves[last] = Line::High;
            }
            if and(inputs[34].get(), inputs[33].get()) {
                let addr = (0..16).map(|i| (inputs[i].get() as usize) << i).sum::<_>();
                for i in 0..16 {
                    outputs[i].set(storage[16 * addr + i]);
                }
            }
        })
    }

    /// Spec:
    /// 0..8: address
    /// 8..16: write value
    /// 16: write
    /// 17: read
    /// 18: clock
    pub fn nor_latch_ram_8() -> Self {
        // Storage:
        // 1 byte as smallest addressable section of memory => 8
        // 256 bytes to fill the address space => 256
        // 8 * 256
        Gate::new_s(19, 256 * 8, 8, |inputs, storage, outputs| {
            if inputs[18].get() {
                let addr = (0..8).map(|i| (inputs[i].get() as usize) << i).sum::<_>();
                if inputs[16].get() {
                    for i in 0..8 {
                        storage[8 * addr + i] = inputs[8 + i].get() | LOW;
                    }
                }
                if inputs[17].get() {
                    for i in 0..8 {
                        outputs[i].set(storage[8 * addr + i]);
                    }
                }
            }
        })
    }

    /// Spec:
    /// 0..16: address
    /// 16..32: write value
    /// 32: write
    /// 33: read
    /// 34: clock
    pub fn nor_latch_ram_16() -> Self {
        // Storage:
        // 2 bytes as smallest addressable section of memory => 16
        // 65536 to fill the available address space => 65536
        // 16 * 65536
        Gate::new_s(35, 65536 * 16, 16, |inputs, storage, outputs| {
            if inputs[34].get() {
                let addr = (0..16).map(|i| (inputs[i].get() as usize) << i).sum::<_>();
                if inputs[32].get() {
                    for i in 0..16 {
                        storage[16 * addr + i] = inputs[16 + i].get() | LOW;
                    }
                }
                if inputs[33].get() {
                    for i in 0..16 {
                        outputs[i].set(storage[16 * addr + i]);
                    }
                }
            }
        })
    }
}