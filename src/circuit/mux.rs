use circuit::{gate::Gate, line::Line, circuit::Circuit};

pub(crate) struct MUX_1_2 {
    i0: usize,
    i1: usize,
    sel: usize,
    output: usize
}

impl MUX_1_2 {
    pub(crate) fn new() -> Self {
        MUX_1_2 {
            i0: 0,
            i1: 0,
            sel: 0,
            output: 0
        }
    }
}

impl Gate for MUX_1_2 {
    fn get_input(&self, i: usize) -> usize {
        match i {
            0 => self.i0,
            1 => self.i1,
            2 => self.sel,
            _ => panic!("Invalid input.")
        }
    }

    fn set_input(&mut self, i: usize, new_i: usize) {
        match i {
            0 => self.i0 = new_i,
            1 => self.i1 = new_i,
            2 => self.sel = new_i,
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

    fn num_outputs(&self) -> usize {
        1
    }

    fn eval(&mut self, lines: &mut Vec<Line>) {
        let tmp = match lines[self.sel] {
            Line::High => lines[self.i1],
            Line::Low => lines[self.i0],
            Line::Disconnected => Line::Disconnected
        };
        lines[self.output] = tmp;
    }
}

pub(crate) struct DMUX_2_2 {
    i0: usize,
    sel: usize,
    o0: usize,
    o1: usize
}

impl DMUX_2_2 {
    pub(crate) fn new() -> Self {
        DMUX_2_2 {
            i0: 0,
            sel: 0,
            o0: 0,
            o1: 0
        }
    }
}

impl Gate for DMUX_2_2 {
    fn get_input(&self, i: usize) -> usize {
        match i {
            0 => self.i0,
            1 => self.sel,
            _ => panic!("Invalid input.")
        }
    }

    fn set_input(&mut self, i: usize, new_i: usize) {
        match i {
            0 => self.i0 = new_i,
            1 => self.sel = new_i,
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

    fn num_outputs(&self) -> usize {
        2
    }

    fn eval(&mut self, lines: &mut Vec<Line>) {
        let tmp = lines[self.i0];
        match lines[self.sel] {
            Line::High => lines[self.o1] = tmp,
            Line::Low => lines[self.o0] = tmp,
            Line::Disconnected => {
                lines[self.o0] = Line::Disconnected;
                lines[self.o1] = Line::Disconnected;
            }
        };
    }
}