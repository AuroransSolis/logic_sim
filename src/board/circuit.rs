use std::rc::Rc;
use std::cell::Cell;

use board::gate::Gate;
use board::line::Line;

#[derive(Debug)]
pub(crate) struct Circuit<T: Gate> {
    pub(crate) gates: Vec<T>,
    pub(crate) lines: Vec<Line>,
    pub(crate) inputs: Vec<usize>,
    pub(crate) outputs: Vec<usize>
}

impl Circuit {
    pub(crate) fn new() -> Self {
        let gates: Vec<Gate> = Vec::new();
        Circuit {
            gates: gates
        }
    }

    pub(crate) fn add_gate(&mut self, gate: Gate) -> usize {
        self.gates.push(gate);
        self.gates.len() - 1
    }

    pub(crate) fn connect_i_single(&mut self, target_gate: usize, target_gate_input: usize,
        tool_gate: usize, tool_gate_output: usize) {
        let new_input = self[tool_gate].get_output(tool_gate_output);
        self[target_gate].set_i(target_gate_input, Some(new_input));
    }

    pub(crate) fn disconnect_i_single(&mut self, target_gate: usize, target_gate_input: usize) {
        self[target_gate].set_i(target_gate_input, None);
    }

    pub(crate) fn update_all_inputs(&mut self,
        update_function: fn(&[Option<Rc<Cell<Option<bool>>>>], &mut [Rc<Cell<Option<bool>>>])) {
        for g in 0..self.gates.len() {
            self[g].update_inputs(update_function);
        }
    }

    pub(crate) fn eval(&mut self, g: usize) {
        self[g].eval();
    }

    pub(crate) fn eval_all(&mut self) {
        for g in 0..self.gates.len() {
            self.eval(g);
        }
    }

    pub(crate) fn eval_all_n_passes(&mut self, passes: usize) {
        for _ in 0..passes {
            self.eval_all();
        }
    }

    // Marked as unsafe because the user has to guarantee that the 'Circuit' is NOT in a board.
    // Otherwise, this function can easily cause memory leaks. The only time that this is safe
    // is if the 'Circuit' has NO connections to other 'Circuit's.
    pub(crate) unsafe fn remove_gate(&mut self, g: usize) {
        let outputs = self[g].get_outputs();
        self.gates.remove(g);
        for g in &mut self.gates {
            let inputs = g.get_inputs();
            for i in 0..inputs.len() {
                if let &Some(ref input) = &inputs[i] {
                    if outputs.iter().any(|output| Rc::ptr_eq(input, output)) {
                        g.set_i(i, None);
                    }
                }
            }
        }
    }
}