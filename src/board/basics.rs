use board::{gate::Gate, line::Line};

impl Gate {
    pub fn and() -> Self {
        Gate::new_ns(2, 1, |inputs, output| output[0].set(inputs[0].get() & inputs[1].get()))
    }

    pub fn or() -> Self {
        Gate::new_ns(2, 1, |inputs, output| output[0].set(inputs[0].get() | inputs[1].get()))
    }

    pub fn xor() -> Self {
        Gate::new_ns(2, 1, |inputs, output| output[0].set(inputs[0].get() ^ inputs[0].get()))
    }


    pub fn nand() -> Self {
        Gate::new_ns(2, 1, |inputs, output| output[0].set(!(inputs[0].get() & inputs[1].get())))
    }


    pub fn nor() -> Self {
        Gate::new_ns(2, 1, |inputs, output| output[0].set(!(inputs[0].get() | inputs[1].get())))
    }


    pub fn xnor() -> Self {
        Gate::new_ns(2, 1, |inputs, output| output[0].set(!(inputs[0].get() ^ inputs[1].get())))
    }

    pub fn not() -> Self {
        Gate::new_ns(1, 1, |input, output| output[0].set(!input[0].get()))
    }

    /// Inputs:
    /// 0 - input line 1
    /// 1 - input line 2
    /// 2 - control line
    /// Control line LOW => output line 1
    /// Control line HIGH => output line 2
    /// Control line DISCONNECTED => output LOW
    pub fn mux_1b_2i1c() -> Self {
        Gate::new_ns(3, 1, |inputs, output| {
            if inputs[2].get().is_disconnected() {
                output[0].set(Line::Low);
            } else {
                if inputs[2].get().is_low() {
                    output[0].set(inputs[0].get());
                } else {
                    output[1].set(inputs[1].get());
                }
            }
        })
    }

    /// Inputs:
    /// 0 - line in
    /// 1 - control line
    /// Control line LOW => output on output line 1
    /// Control line HIGH => output on output line 2
    /// Control line DISCONNECTED => output DISCONNECTED on both output lines
    pub fn dmux_1b_2o1c() -> Self {
        Gate::new_ns(2, 2, |inputs, outputs| {
            if inputs[1].get().is_disconnected() {
                outputs[0].set(Line::Disconnected);
                outputs[1].set(Line::Disconnected);
            } else {
                if inputs[1].get().is_low() {
                    outputs[0].set(inputs[0].get());
                    outputs[1].set(Line::Low);
                } else {
                    outputs[0].set(Line::Low);
                    outputs[1].set(inputs[0].get());
                }
            }
        })
    }
}