use std::ops::{Index, IndexMut};

use board::{circuit::Circuit, gate::Gate};

#[derive(Debug)]
pub(crate) struct Board {
    circuits: Vec<Circuit>
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
            circuits: Vec::new()
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
        for (i, c) in self.circuits.iter_mut().enumerate() {
            c.eval_all_n_passes(passes);
        }
    }

    pub(crate) fn eval_all_n_passes_n_passes(&mut self, c_passes: usize, g_passes: usize) {
        for _ in 0..c_passes {
            for c in self.circuits.iter_mut() {
                c.eval_all_n_passes(g_passes);
            }
        }
    }

    pub(crate) fn make_inter_circuit_i1_connection(&mut self, target_circuit: usize,
        target_gate: usize, tool_circuit: usize, tool_gate: usize) {
        let ptr = self[tool_circuit].get_output_ptr(tool_gate);
        self[target_circuit].connect_i1_ptr(target_gate, ptr);
    }

    pub(crate) fn make_inter_circuit_i2_connection(&mut self, target_circuit: usize,
        target_gate: usize, tool_circuit: usize, tool_gate: usize) {
        let ptr = self[tool_circuit].get_output_ptr(tool_gate);
        self[target_circuit].connect_i2_ptr(target_gate, ptr);
    }

    pub(crate) fn remove_gate(&mut self, target_circuit: usize, target_gate: usize) {
        let target_ptr = self[target_circuit][target_gate].get_output_ptr();
        for ci in 0..self.circuits.len() {
            for gi in (0..self[ci].gates.len())
                .filter(|&gi| gi != target_gate && ci != target_circuit) {
                if let Some(i1) = self[ci][gi].get_i1() {
                    if i1 == target_ptr {
                        self[ci][gi].disconnect_i1();
                    }
                }
                if let Some(i2) = self[ci][gi].get_i2() {
                    if i2 == target_ptr {
                        self[ci][gi].disconnect_i2();
                    }
                }
            }
        }
        self[target_circuit].gates.remove(target_gate);
    }

    pub(crate) fn remove_circuit(&mut self, target_circuit: usize) {
        for gi in 0..self[target_circuit].gates.len() {
            let ptr = self[target_circuit][gi].get_output();
            for ci in (0..self.circuits.len()).filter(|&ci| ci != target_circuit) {
                for other_gi in 0..self[ci].gates.len() {
                    if let Some(i1) = self[ci][other_gi].get_i1() {
                        if i1 == ptr {
                            self[ci][other_gi].disconnect_i1();
                        }
                    }
                    if let Some(i2) = self[ci][other_gi].get_i2() {
                        if i2 == ptr {
                            self[ci][other_gi].disconnect_i2();
                        }
                    }
                }
            }
        }
        self.circuits.remove(target_circuit);
    }
}