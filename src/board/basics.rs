use board::{gate::Gate, line::Line, circuit::Circuit};

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