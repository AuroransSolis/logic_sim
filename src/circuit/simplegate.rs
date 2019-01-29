use circuit::line::{Line, HIGH, LOW, DISCONNECTED, not, and, or, xor, nand, nor, xnor};
use circuit::circuit::Circuit;
use circuit::gate::Gate;

pub(crate) struct SimpleGate {
    i0: usize,
    i1: usize,
    function: fn(Line, Line) -> Line,
    output: usize
}

impl SimpleGate {
    pub(crate) fn and(i0: usize, i1: usize, output: usize) -> Self {
        SimpleGate {
            i0,
            i1,
            function: |i0, i1| and(i0, i1),
            output
        }
    }

    pub(crate) fn or(i0: usize, i1: usize, output: usize) -> Self {
        SimpleGate {
            i0,
            i1,
            function: |i0, i1| or(i0, i1),
            output
        }
    }

    pub(crate) fn xor(i0: usize, i1: usize, output: usize) -> Self {
        SimpleGate {
            i0,
            i1,
            function: |i0, i1| xor(i0, i1),
            output
        }
    }

    pub(crate) fn nand() -> Self {
        SimpleGate {
            i0: usize,
            i1: usize,
            function: |i0, i1| nand(i0, i1),
            output: usize
        }
    }

    pub(crate) fn nor() -> Self {
        SimpleGate {
            i0: usize,
            i1: usize,
            function: |i0, i1| nor(i0, i1),
            output: usize
        }
    }

    pub(crate) fn xnor() -> Self {
        SimpleGate {
            i0: usize,
            i1: usize,
            function: |i0, i1| xnor(i0, i1),
            output: usize
        }
    }
}

impl Gate for SimpleGate {
    fn get_input(&self, i: usize) -> usize {
        match i {
            0 => self.i0,
            1 => self.i1,
            _ => panic!("Invalid input.")
        }
    }

    fn set_input(&mut self, i: usize, new_i: usize) {
        match i {
            0 => self.i0 = new_i,
            1 => self.i1 = new_i,
            _ => panic!("Attempting to set invalid input.")
        }
    }

    fn num_inputs(&self) -> usize {
        2
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

    fn eval(&self, circuit: &mut Circuit) {
        let tmp = self.function(circuit.lines[self.i0], circuit.lines[self.i1]);
        circuit.lines[self.output] = tmp;
    }
}