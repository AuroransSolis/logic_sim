use board::gate::Gate;

impl Gate {
    pub(crate) fn and() -> Self {
        Gate::new_ns(2, 1, |inputs, output| {
            if let (Some(rcc_opt_b1), Some(rcc_opt_b2)) = (inputs[0].as_ref(), inputs[1].as_ref()) {
                if let (Some(b1), Some(b2)) = (rcc_opt_b1.get(), rcc_opt_b2.get()) {
                    output[0].set(Some(b1 && b2));
                } else {
                    output[0].set(None);
                }
            } else {
                output[0].set(None);
            }
        })
    }


    pub(crate) fn or() -> Self {
        Gate::new_ns(2, 1, |inputs, output| {
            if let (Some(rcc_opt_b1), Some(rcc_opt_b2)) = (inputs[0].as_ref(), inputs[1].as_ref()) {
                if let (Some(b1), Some(b2)) = (rcc_opt_b1.get(), rcc_opt_b2.get()) {
                    output[0].set(Some(b1 || b2));
                } else {
                    output[0].set(None);
                }
            } else {
                output[0].set(None);
            }
        })
    }


    pub(crate) fn xor() -> Self {
        Gate::new_ns(2, 1, |inputs, output| {
            if let (Some(rcc_opt_b1), Some(rcc_opt_b2)) = (inputs[0].as_ref(), inputs[1].as_ref()) {
                if let (Some(b1), Some(b2)) = (rcc_opt_b1.get(), rcc_opt_b2.get()) {
                    output[0].set(Some(b1 != b2));
                } else {
                    output[0].set(None);
                }
            } else {
                output[0].set(None);
            }
        })
    }


    pub(crate) fn nand() -> Self {
        Gate::new_ns(2, 1, |inputs, output| {
            if let (Some(rcc_opt_b1), Some(rcc_opt_b2)) = (inputs[0].as_ref(), inputs[1].as_ref()) {
                if let (Some(b1), Some(b2)) = (rcc_opt_b1.get(), rcc_opt_b2.get()) {
                    output[0].set(Some(!(b1 && b2)));
                } else {
                    output[0].set(None);
                }
            } else {
                output[0].set(None);
            }
        })
    }


    pub(crate) fn nor() -> Self {
        Gate::new_ns(2, 1, |inputs, output| {
            if let (Some(rcc_opt_b1), Some(rcc_opt_b2)) = (inputs[0].as_ref(), inputs[1].as_ref()) {
                if let (Some(b1), Some(b2)) = (rcc_opt_b1.get(), rcc_opt_b2.get()) {
                    output[0].set(Some(!(b1 || b2)));
                } else {
                    output[0].set(None);
                }
            } else {
                output[0].set(None);
            }
        })
    }


    pub(crate) fn xnor() -> Self {
        Gate::new_ns(2, 1, |inputs, output| {
            if let (Some(rcc_opt_b1), Some(rcc_opt_b2)) = (inputs[0].as_ref(), inputs[1].as_ref()) {
                if let (Some(b1), Some(b2)) = (rcc_opt_b1.get(), rcc_opt_b2.get()) {
                    output[0].set(Some(b1 == b2));
                } else {
                    output[0].set(None);
                }
            } else {
                output[0].set(None);
            }
        })
    }

    pub(crate) fn not() -> Self {
        Gate::new_ns(1, 1, |input, output| {
            if let Some(rcc_opt_b) = input[0].as_ref() {
                if let Some(b) = rcc_opt_b.get() {
                    output[0].set(Some(!b));
                } else {
                    output[0].set(None);
                }
            } else {
                output[0].set(None);
            }
        })
    }

    // Inputs:
    // 0 - input line 1
    // 1 - input line 2
    // 2 - control line
    pub(crate) fn mux_1b_2i1c() -> Self {
        Gate::new_ns(3, 1, |inputs, output| {
            if let (Some(opt_ctrl_line), Some(opt_line_1), Some(opt_line_2))
                = (&inputs[0], &inputs[1], &inputs[2]) {
                if let Some(true) = opt_ctrl_line.get() {
                    output[0].set(opt_line_1.get());
                } else if let Some(false) = opt_ctrl_line.get() {
                    output[0].set(opt_line_2.get());
                } else {
                    output[0].set(None);
                }
            } else {
                output[0].set(None);
            }
        })
    }

    // Inputs:
    // 0 - line in
    // 1 - control line
    pub(crate) fn dmux_1b_2o1c() -> Self {
        Gate::new_ns(2, 2, |inputs, outputs| {
            if let (Some(opt_line_in), Some(opt_ctrl_line))
                = (&inputs[0], &inputs[1]) {
                if let Some(true) = opt_ctrl_line.get() {
                    outputs[0].set(opt_line_in.get());
                    outputs[1].set(Some(false));
                } else if let Some(false) = opt_ctrl_line.get() {
                    outputs[1].set(opt_line_in.get());
                    outputs[0].set(Some(false));
                } else {
                    outputs[0].set(None);
                    outputs[1].set(None);
                }
            } else {
                outputs[0].set(None);
                outputs[1].set(None);
            }
        })
    }
}