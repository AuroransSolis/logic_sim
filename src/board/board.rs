use std::ops::{Index, IndexMut};
use std::rc::Rc;

use board::{circuit::Circuit};

enum Wire {
    Intracircuit{target_circuit: usize, target_gate: usize, tool_gate: usize},
    Intercircuit{target_circuit: usize, target_gate: usize, tool_circuit: usize, tool_gate: usize}
}

#[derive(Debug)]
pub(crate) struct Board {
    circuits: Vec<Circuit>,
    //wires: Vec<Wire>
}

impl Index<usize> for Board {
    type Output = Circuit;

    fn index(&self, idx: usize) -> &Circuit {
        &self.circuits[idx]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, idx: usize) -> &mut Circuit {
        &mut self.circuits[idx]
    }
}

impl Board {
    pub(crate) fn new() -> Self {
        Board {
            circuits: Vec::new(),
            //wires: Vec::new()
        }
    }

    pub(crate) fn add_circuit(&mut self, circuit: Circuit) -> usize {
        self.circuits.push(circuit);
        self.circuits.len() - 1
    }

    pub(crate) fn eval(&mut self, c: usize) {
        self[c].eval_all();
    }

    pub(crate) fn eval_circuit_n_passes(&mut self, c: usize, passes: usize) {
        self[c].eval_all_n_passes(passes);
    }

    pub(crate) fn eval_all_n_passes(&mut self, passes: usize) {
        for c in &mut self.circuits {
            c.eval_all_n_passes(passes);
        }
    }

    pub(crate) fn eval_all_n_passes_n_passes(&mut self, c_passes: usize, g_passes: usize) {
        for _ in 0..c_passes {
            for c in &mut self.circuits {
                c.eval_all_n_passes(g_passes);
            }
        }
    }

    pub(crate) fn make_inter_circuit_i_connection(&mut self, target_circuit: usize,
        target_gate: usize, target_gate_input: usize, tool_circuit: usize, tool_gate: usize,
        tool_gate_output: usize) {
        let output_rcc = self[tool_circuit][tool_gate].get_output(tool_gate_output);
        self[target_circuit][target_gate].set_i(target_gate_input, Some(output_rcc));
    }

    pub(crate) fn remove_gate(&mut self, target_circuit: usize, target_gate: usize) {
        let outputs = self[target_circuit][target_gate].get_outputs();
        unsafe {
            self[target_circuit].remove_gate(target_gate);
        }
        for c in &mut self.circuits {
            for g in &mut c.gates {
                let inputs = g.get_inputs();
                for i in 0..inputs.len() {
                    if let Some(ref input) = inputs[i] {
                        if outputs.iter().any(|output| Rc::ptr_eq(&input, output)) {
                            g.set_i(i, None);
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn remove_circuit(&mut self, target_circuit: usize) {
        let mut outputs = Vec::new();
        for g in &self[target_circuit].gates {
            outputs.append(&mut g.get_outputs());
        }
        self.circuits.remove(target_circuit);
        for c in &mut self.circuits {
            for g in &mut c.gates {
                let inputs = g.get_inputs();
                for i in 0..inputs.len() {
                    if let Some(ref input) = inputs[i] {
                        if outputs.iter().any(|output| Rc::ptr_eq(&input, output)) {
                            g.set_i(i, None);
                        }
                    }
                }
            }
        }
    }
}