use circuit::{gate::Gate, line::Line, circuit::Circuit};

pub struct Source {
    source: Line,
    output: usize
}

impl Source {
    pub(crate) fn new_low() -> Self {
        Source {
            source: Line::Low,
            output: 0
        }
    }

    pub(crate) fn new_high() -> Self {
        Source {
            source: Line::High,
            output: 0
        }
    }
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

    fn num_outputs(&self) -> usize {
        1
    }

    fn eval(&mut self, lines: &mut Vec<Line>) {
        lines[self.output] = self.source;
    }
}

pub struct Sink {
    i0: usize,
    pub(crate) sink: Line
}

impl Sink {
    pub(crate) fn new() -> Self {
        Sink {
            i0: 0,
            sink: Line::Disconnected
        }
    }
}

impl Gate for Sink {
    fn get_input(&self, _i: usize) -> usize {
        self.i0
    }

    fn set_input(&mut self, _i: usize, new_i: usize) {
        self.i0 = new_i;
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

    fn num_outputs(&self) -> usize {
        0
    }

    fn eval(&mut self, lines: &mut Vec<Line>) {
        let tmp = lines[self.i0];
        self.sink = tmp;
    }
}

pub struct Inverter {
    i0: usize,
    o0: usize
}

impl Inverter {
    pub(crate) fn new() -> Self {
        Inverter {
            i0: 0,
            o0: 0
        }
    }
}

impl Gate for Inverter {
    fn get_input(&self, _i: usize) -> usize {
        self.i0
    }

    fn set_input(&mut self, _i: usize, new_i: usize) {
        self.i0 = new_i;
    }

    fn num_inputs(&self) -> usize {
        1
    }

    fn get_output(&self, _o: usize) -> usize {
        self.o0
    }

    fn set_output(&mut self, _o: usize, new_o: usize) {
        self.o0 = new_o;
    }

    fn num_outputs(&self) -> usize {
        1
    }

    fn eval(&mut self, lines: &mut Vec<Line>) {
        lines[self.o0] = !lines[self.i0];
    }
}