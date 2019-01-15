use board::{gate::Gate, line::Line, circuit::Circuit};

pub(crate) struct MUX_1_2 {
    i0: usize,
    i1: usize,
    sel: usize,
    output: usize
}

impl MUX_1_2 {
    pub(crate) fn new(i0: usize, i1: usize, sel: usize, output: usize) -> Self {
        MUX_1_2 {
            i0,
            i1,
            sel,
            output
        }
    }
}

impl Gate for MUX_1_2 {
    fn get_input(&self, i: usize) -> usize {
        match i {
            0 => i0,
            1 => i1,
            2 => sel,
            _ => panic!("Invalid input.")
        }
    }

    fn set_input(&mut self, i: usize, new_i: usize) {
        match i {
            0 => i0 = new_i,
            1 => i1 = new_i,
            2 => sel = new_i,
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
            _ => panic!("Attempting to set invalid output.")
        }
    }

    fn num_ouputs(&self) -> usize {
        1
    }

    fn eval(&self, circuit: &mut Circuit<Gate>) {
        let tmp = match circuit.lines[self.sel] {
            HIGH => circuit.lines[self.i1],
            LOW => circuit.lines[self.i0],
            DISCONNECTED => DISCONNECTED
        };
        circuit.lines[self.output] = tmp;
    }
}

pub(crate) struct DMUX_2_2 {
    i0: usize,
    sel: usize,
    o0: usize,
    o1: usize
}

impl DMUX_2_2 {
    pub(crate) fn new(i0: usize, sel: usize, o0: usize, o1: usize) -> Self {
        DMUX_2_2 {
            i0,
            sel,
            o0,
            o1
        }
    }
}

impl Gate for DMUX_2_2 {
    fn get_input(&self, i: usize) -> usize {
        match i {
            0 => i0,
            1 => sel,
            _ => panic!("Invalid input.")
        }
    }

    fn set_input(&mut self, i: usize, new_i: usize) {
        match i {
            0 => i0 = new_i,
            1 => sel = new_i,
            _ => panic!("Attempted to set invalid input.")
        }
    }

    fn num_inputs(&self) -> usize {
        2
    }

    fn get_output(&self, o: usize) -> usize {
        match o {
            0 => self.o0,
            1 => self.o1,
            _ => panic!("Invalid output.")
        }
    }

    fn set_output(&mut self, o: usize, new_o: usize) {
        match o {
            0 => self.o0 = new_o,
            1 => self.o1 = new_o,
            _ => panic!("Attempting to set invalid output.")
        }
    }

    fn num_ouputs(&self) -> usize {
        2
    }

    fn eval(&self, circuit: &mut Circuit<Gate>) {
        let tmp = circuit.lines[self.i0];
        match circuit.lines[self.sel] {
            HIGH => circuit.lines[self.o1] = tmp,
            LOW => circuit.lines[self.o0] = tmp,
            DISCONNECTED => DISCONNECTED
        };
    }
}

pub(crate) struct Source {
    source: Line,
    output: usize
}

impl Gate for Source {
    fn get_input(&self, _i: usize) -> usize {
        panic!("Sources have no inputs.");
    }

    fn set_input(&mut self, _i: usize, _new_i: usize) {
        panic!("Sources have no inputs.");
    }

    fn num_inputs(&self) -> usize {
        0
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
            _ => panic!("Attempting to set invalid output.")
        }
    }

    fn num_ouputs(&self) -> usize {
        1
    }

    fn eval(&self, circuit: &mut Circuit<Gate>) {
        circuit.lines[self.output] = self.source;
    }
}

pub(crate) struct Sink {
    i0: usize,
    pub(crate) sink: Line
}

impl Gate for Sink {
    fn get_input(&self, _i: usize) -> usize {
        match o {
            0 => self.output,
            _ => panic!("Invalid input.")
        }
    }

    fn set_input(&mut self, _i: usize, _new_i: usize) {
        match o {
            0 => self.output = new_o,
            _ => panic!("Attempting to set invalid output.")
        }
    }

    fn num_inputs(&self) -> usize {
        1
    }

    fn get_output(&self, o: usize) -> usize {
        panic!("Sinks have no outputs.");
    }

    fn set_output(&mut self, o: usize, new_o: usize) {
        panic!("Sinks have no outputs.");
    }

    fn num_ouputs(&self) -> usize {
        0
    }

    fn eval(&self, circuit: &mut Circuit<Gate>) {
        unsafe { *(&self.sink as *const Line) } = circuit.lines[self.output];
    }
}