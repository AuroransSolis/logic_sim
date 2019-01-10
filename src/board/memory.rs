use board::{gate::Gate, line::Line};

impl Gate {
    /// Spec:
    /// 0: write 0 to master
    /// 1: write 1 to master
    /// 2: clock
    pub fn master_slave_flip_flop() -> Self {
        // Third `Line` is used as a flag to signify whether the write has been done on the trailing
        // edge of the clock signal.
        Gate::new_s(3, 2, 1, |inputs, storage, output| {
            if inputs[2].get().is_high() {
                storage[1] = Line::Low;
                if !(inputs[0].get().is_high() && inputs[1].get().is_high()) {
                    if inputs[0].get().is_high() {
                        storage[0] = Line::Low;
                    } else if inputs[1].get().is_high() {
                        storage[0] = Line::High;
                    }
                }
            } else if inputs[2].get().is_low() && storage[1].is_low() {
                output[0].set(storage[0]);
                storage[1] = Line::High;
            }
        })
    }

    /// Spec:
    /// 0: set latch low
    /// 1: set latch high
    pub fn nor_latch() -> Self {
        Gate::new_ns(2, 1, |inputs, output| {
            if !(inputs[0].get().is_high() && inputs[1].get().is_high()) {
                if inputs[0].get().is_high() {
                    output[0].set(Line::Low);
                } else if inputs[1].get().is_high() {
                    output[0].set(Line::High);
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
            if inputs[18].get().is_high() {
                if inputs[16].get().is_high() {
                    let last_ind = storage.len() - 1;
                    storage[last_ind] = Line::Low;
                    let addr = {
                        let mut total = 0;
                        for i in 0..8 {
                            if inputs[i].get().is_high() {
                                total += 1 << i;
                            }
                        }
                        total
                    };
                    for i in 0..8 {
                        // The `| Line::Low` below is to ensure that storage is always
                        // `Line::High` or `Line::Low`, since
                        // `Line::Disconnected | Line::Low == Line::Low`.
                        storage[8 * addr + i] = inputs[8 + i].get() | Line::Low;
                    }
                }
            } else if inputs[18].get().is_low() && storage[storage.len() - 1].is_low() {
                let (masters, slaves) = storage.split_at_mut(256 * 8);
                slaves.copy_from_slice(&*masters);
                let last = slaves.len() - 1;
                slaves[last] = Line::High;
            }
            if inputs[17].get().is_high() {
                let addr = {
                    let mut total = 0;
                    for i in 0..8 {
                        if inputs[i].get().into() {
                            total += 1 << i;
                        }
                    }
                    total
                };
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
            if inputs[34].get().is_high() {
                if inputs[32].get().is_high() {
                    let last_ind = storage.len() - 1;
                    storage[last_ind] = Line::Low;
                    let addr = {
                        let mut total = 0;
                        for i in 0..16 {
                            if inputs[i].get().is_high() {
                                total += 1 << i;
                            }
                        }
                        total
                    };
                    for i in 0..16 {
                        storage[16 * addr + i] = inputs[16 + i].get() | Line::Low;
                    }
                }
            } else if inputs[34].get().is_low() && inputs[32].get().is_high() {
                let (masters, slaves) = storage.split_at_mut(16 * 65536);
                slaves.copy_from_slice(&*masters);
                let last = slaves.len() - 1;
                slaves[last] = Line::High;
            }
            if inputs[33].get().is_high() {
                let addr = {
                    let mut total = 0;
                    for i in 0..16 {
                        if inputs[i].get().is_high() {
                            total += 1 << i;
                        }
                    }
                    total
                };
                for i in 0..16 {
                    outputs[i].set(storage[16 * addr + i])
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
            if inputs[18].get().is_high() {
                let addr = {
                    let mut total = 0;
                    for i in 0..8 {
                        if inputs[i].get().into() {
                            total += 1 << i;
                        }
                    }
                    total
                };
                if inputs[16].get().is_high() {
                    for i in 0..8 {
                        storage[8 * addr + i] = inputs[8 + i].get() | Line::Low;
                    }
                }
                if inputs[17].get().is_high() {
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
            if inputs[34].get().is_high() {
                let addr = {
                    let mut total = 0;
                    for i in 0..16 {
                        if inputs[i].get().into() {
                            total += 1 << i;
                        }
                    }
                    total
                };
                if inputs[32].get().is_high() {
                    for i in 0..16 {
                        storage[16 * addr + i] = inputs[16 + i].get() | Line::Low;
                    }
                }
                if inputs[33].get().is_high() {
                    for i in 0..16 {
                        outputs[i].set(storage[16 * addr + i]);
                    }
                }
            }
        })
    }
}