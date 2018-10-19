use std::ops::{Index, IndexMut};

use board::{circuit::Circuit, gate::Gate};

#[derive(Copy, Clone)]
pub(crate) struct C2C {
    source_circuit: usize,
    sc_index: usize,
    destination_circuit: usize,
    dc_index: usize
}

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
    fn new() -> Self {
        Board {
            circuits: Vec::new()
        }
    }

    fn add_circuit(&mut self, circuit: Circuit) -> usize {
        self.circuits.push(circuit);
        self.circuits.len() - 1
    }

    fn eval(&mut self, c: usize) {
        self[c].eval_all();
    }

    fn eval_circuit_n_passes(&mut self, c: usize, passes: usize) {
        self[c].eval_all_n_passes(passes);
    }

    fn eval_all_n_passes(&mut self, passes: usize) {
        for c in self.circuits.iter_mut() {
            c.eval_all_n_passes(passes);
        }
    }

    fn eval_all_n_passes_n_passes(&mut self, c_passes: usize, g_passes: usize) {
        for _ in 0..c_passes {
            for c in self.circuits.iter_mut() {
                c.eval_all_n_passes(g_passes);
            }
        }
    }

    fn make_inter_circuit_i1_connection(&mut self, c1: usize, g1: usize, c2: usize, g2: usize) {
        let ptr = self[c2].get_output_ptr(g2);
        self[c1].connect_i1_ptr(g1, ptr);
    }
}