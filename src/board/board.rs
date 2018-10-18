use board::{circuit::Circuit, gate::Gate};

#[derive(Copy, Clone)]
pub(crate) struct C2C {
    source_circuit: usize,
    sc_index: usize,
    destination_circuit: usize,
    dc_index: usize
}

pub(crate) struct Board {
    connections: Vec<C2C>,
    circuits: Vec<Circuit>
}

impl Board {
    fn new() -> Self {
        Board {
            connections: Vec::new(),
            circuits: Vec::new()
        }
    }

    fn add_circuit(&mut self, circuit: Circuit) {
        self.circuits.push(circuit);
    }

    fn add_gate(&mut self, circuit: usize, gate: Gate) {
        self.circuits[circuit].add_gate(gate);
    }

    fn eval_circuit(&mut self, circuit: usize) {
        if let Some(pos) = self.connections.iter().position(|&c2c| c2c.source_circuit == circuit) {
            self.eval_circuit(pos);
            // How do I now let the gate in the target circuit know what its input from the other circuit is?
        }
        self.circuits[circuit].eval_all();
    }
}