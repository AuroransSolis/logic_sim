use std::ops::Index;
use std::ops::IndexMut;

use board::gate::Gate::{self, *};

#[derive(Debug)]
pub(crate) struct Circuit {
    gates: Vec<Gate>
}

impl Index<usize> for Circuit {
    type Output = Gate;

    fn index(&self, idx: usize) -> &Gate {
        &self.gates[idx]
    }
}

impl IndexMut<usize> for Circuit {
    fn index_mut(&mut self, idx: usize) -> &mut Gate {
        &mut self.gates[idx]
    }
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

    pub(crate) fn connect_i1(&mut self, target_gate: usize, tool_gate: usize) {
        self[target_gate].connect_i1(tool_gate);
    }

    pub(crate) fn connect_i2(&mut self, target_gate: usize, tool_gate: usize) {
        self[target_gate].connect_i2(tool_gate);
    }

    pub(crate) fn disconnect_i1(&mut self, g: usize) {
        self[g].disconnect_i1();
    }

    pub(crate) fn disconnect_i2(&mut self, g: usize) {
        self[g].disconnect_i2();
    }

    pub(crate) fn set_high(&mut self, g: usize) {
        self[g].set_high();
    }

    pub(crate) fn set_low(&mut self, g: usize) {
        self[g].set_low();
    }

    pub(crate) fn get_output(&self, g: usize) -> Option<bool> {
        self[g].get_output()
    }
    
    // Checks to see if the input(s) are both Some(ref). If they are, the output is set to the
    // default value (false).
    pub(crate) fn update_inputs(&mut self, g: usize) {
        match self[g] {
            And{i1, i2, output} => {
                if i1.is_some() && i2.is_some() && self[i1.unwrap()].get_output().is_some()
                    && self[i2.unwrap()].get_output().is_some() {
                    self.eval(g);
                } else if i1.is_some() && i2.is_some() {
                    self[g] = And{i1, i2, output: Some(false)}
                }
            },
            Or{i1, i2, output} => {
                if i1.is_some() && i2.is_some() && self[i1.unwrap()].get_output().is_some()
                    && self[i2.unwrap()].get_output().is_some() {
                    self.eval(g);
                } else if i1.is_some() && i2.is_some() {
                    self[g] = Or{i1, i2, output: Some(false)}
                }
            },
            Xor{i1, i2, output} => {
                if i1.is_some() && i2.is_some() && self[i1.unwrap()].get_output().is_some()
                    && self[i2.unwrap()].get_output().is_some() {
                    self.eval(g);
                } else if i1.is_some() && i2.is_some() {
                    self[g] = Xor{i1, i2, output: Some(false)}
                }
            },
            Nand{i1, i2, output} => {
                if i1.is_some() && i2.is_some() && self[i1.unwrap()].get_output().is_some()
                    && self[i2.unwrap()].get_output().is_some() {
                    self.eval(g);
                } else if i1.is_some() && i2.is_some() {
                    self[g] = Nand{i1, i2, output: Some(false)}
                }
            },
            Nor{i1, i2, output} => {
                if i1.is_some() && i2.is_some() && self[i1.unwrap()].get_output().is_some()
                    && self[i2.unwrap()].get_output().is_some() {
                    self.eval(g);
                } else if i1.is_some() && i2.is_some() {
                    self[g] = Nor{i1, i2, output: Some(false)}
                }
            },
            Xnor{i1, i2, output} => {
                if i1.is_some() && i2.is_some() && self[i1.unwrap()].get_output().is_some()
                    && self[i2.unwrap()].get_output().is_some() {
                    self.eval(g);
                } else if i1.is_some() && i2.is_some() {
                    self[g] = Xnor{i1, i2, output: Some(false)}
                }
            },
            Not{i1, output} => {
                if i1.is_some() && self[i1.unwrap()].get_output().is_some() {
                    self.eval(g);
                } else if i1.is_some() {
                    self[g] = Not{i1, output: Some(false)};
                }
            },
            _ => {}
        }
    }

    pub(crate) fn update_all_inputs(&mut self) {
        for g in 0..self.gates.len() {
            self.update_inputs(g);
        }
    }
    
    pub(crate) fn eval(&mut self, g: usize) {
        match self[g] {
            And{i1, i2, output} => {
                if i1.is_none() || i2.is_none() {
                    self[g] = And{i1, i2, output: None};
                } else if self[i1.unwrap()].get_output().is_none() || self[i2.unwrap()]
                    .get_output().is_none() {
                    self[g] = And{i1, i2, output: None};
                } else {
                    self[g] = And{i1, i2, output: Some(self[i1.unwrap()].get_output().unwrap()
                        && self[i2.unwrap()].get_output().unwrap())};
                }
            },
            Or{i1, i2, output} => {
                if i1.is_none() || i2.is_none() {
                    self[g] = Or{i1, i2, output: None};
                } else if self[i1.unwrap()].get_output().is_none() || self[i2.unwrap()]
                    .get_output().is_none() {
                    self[g] = Or{i1, i2, output: None};
                } else {
                    self[g] = Or{i1, i2, output: Some(self[i1.unwrap()].get_output().unwrap()
                        || self[i2.unwrap()].get_output().unwrap())};
                }
            },
            Xor{i1, i2, output} => {
                if i1.is_none() || i2.is_none() {
                    self[g] = Xor{i1, i2, output: None};
                } else if self[i1.unwrap()].get_output().is_none() || self[i2.unwrap()]
                    .get_output().is_none() {
                    self[g] = Xor{i1, i2, output: None};
                } else {
                    self[g] = Xor{i1, i2, output: Some(self[i1.unwrap()].get_output().unwrap()
                        != self[i2.unwrap()].get_output().unwrap())};
                }
            },
            Not{i1, output} => {
                if i1.is_none() {
                    self[g] = Not{i1, output: None};
                } else if self[i1.unwrap()].get_output().is_none() {
                    self[g] = Not{i1, output: None};
                } else {
                    self[g] = Not{i1, output: Some(!self[i1.unwrap()].get_output().unwrap())};
                }
            },
            Nand{i1, i2, output} => {
                if i1.is_none() || i2.is_none() {
                    self[g] = Nand{i1, i2, output: None};
                } else if self[i1.unwrap()].get_output().is_none() || self[i2.unwrap()]
                    .get_output().is_none() {
                    self[g] = Nand{i1, i2, output: None};
                } else {
                    self[g] = Nand{i1, i2, output: Some(!(self[i1.unwrap()].get_output().unwrap()
                        && self[i2.unwrap()].get_output().unwrap()))};
                }
            },
            Nor{i1, i2, output} => {
                if i1.is_none() || i2.is_none() {
                    self[g] = Nor{i1, i2, output: None};
                } else if self[i1.unwrap()].get_output().is_none() || self[i2.unwrap()]
                    .get_output().is_none() {
                    self[g] = Nor{i1, i2, output: None};
                } else {
                    self[g] = Nor{i1, i2, output: Some(!(self[i1.unwrap()].get_output().unwrap()
                        || self[i2.unwrap()].get_output().unwrap()))};
                }
            },
            Xnor{i1, i2, output} => {
                if i1.is_none() || i2.is_none() {
                    self[g] = Xnor{i1, i2, output: None};
                } else if self[i1.unwrap()].get_output().is_none() || self[i2.unwrap()]
                    .get_output().is_none() {
                    self[g] = Xnor{i1, i2, output: None};
                } else {
                    self[g] = Xnor{i1, i2, output: Some(self[i1.unwrap()].get_output().unwrap()
                        != self[i2.unwrap()].get_output().unwrap())};
                }
            },
            _ => {}
        }
    }

    pub(crate) fn eval_all(&mut self) {
        for g in 0..self.gates.len() {
            self.eval(g);
        }
        for g in 0..self.gates.len() {
            self.eval(g);
        }
    }
}