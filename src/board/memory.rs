use board::gate::Gate;

impl Gate {
    // Spec:
    // 0..8: address
    // 8..16: write value
    // 16: write
    // 17: read
    // 18: clock
    pub(crate) fn ram_8() -> Self {
        Gate::new_s(19, 256 * 8, 8, |inputs, storage, outputs| {
            if None == inputs[18] {
                return;
            } else if let Some(opt_clk) = &inputs[18] {
                if opt_clk.get().is_none() {
                    return;
                } else if let Some(false) = opt_clk.get() {
                    return;
                }
            }
            if inputs.contains(&None) {
                return;
            }
            for i in 0..18 {
                if inputs[i].as_ref().unwrap().get().is_none() {
                    return;
                }
            }
            let mut addr_inputs = [false; 8];
            let mut write_val = [false; 8];
            for i in 0..8 {
                addr_inputs[i] = inputs[i].as_ref().unwrap().get().unwrap();
                write_val[i] = inputs[8 + i].as_ref().unwrap().get().unwrap();
            }
            let write = inputs[16].as_ref().unwrap().get().unwrap();
            let read = inputs[17].as_ref().unwrap().get().unwrap();
            let addr = from_bool_8(&addr_inputs);
            if write {
                for i in 0..8 {
                    storage[8 * addr + i] = write_val[i];
                }
            }
            if read {
                for i in 0..8 {
                    outputs[i].set(Some(storage[8 * addr + i]));
                }
            }
        })
    }

    // Spec:
    // 0..16: address
    // 16..32: write value
    // 32: write
    // 33: read
    // 34: clock
    pub(crate) fn ram_16() -> Self {
        Gate::new_s(35, 256 * 8, 16, |inputs, storage, outputs| {
            if None == inputs[34] {
                return;
            } else if let Some(opt_clk) = &inputs[18] {
                if opt_clk.get().is_none() {
                    return;
                } else if let Some(false) = opt_clk.get() {
                    return;
                }
            }
            if inputs.contains(&None) {
                return;
            }
            for i in 0..35 {
                if inputs[i].as_ref().unwrap().get().is_none() {
                    return;
                }
            }
            let mut addr_inputs = [false; 16];
            let mut write_val = [false; 16];
            for i in 0..16 {
                addr_inputs[i] = inputs[i].as_ref().unwrap().get().unwrap();
                write_val[i] = inputs[16 + i].as_ref().unwrap().get().unwrap();
            }
            let write = inputs[32].as_ref().unwrap().get().unwrap();
            let read = inputs[33].as_ref().unwrap().get().unwrap();
            let addr = from_bool_16(&addr_inputs);
            if write {
                for i in 0..16 {
                    storage[16 * addr + i] = write_val[i];
                }
            }
            if read {
                for i in 0..16 {
                    outputs[i].set(Some(storage[16 * addr + i]));
                }
            }
        })
    }
}

fn from_bool_8(other: &[bool; 8]) -> usize {
    let mut total = 0;
    for i in 0..8 {
        if other[i] {
            total += 1 << i;
        }
    }
    total
}


fn from_bool_16(other: &[bool; 16]) -> usize {
    let mut total = 0;
    for i in 0..16 {
        if other[i] {
            total += 1 << i;
        }
    }
    total
}