use std::ops::Index;
use std::ops::IndexMut;

use crate::gate::Gate::{self, *};

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

    pub(crate) fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    pub(crate) fn remove_gate(&mut self, ind: usize) {
        for g in self.gates.iter_mut() {
            let (i1, i2) = (g.get_i1(), g.get_i2());
            if let Some(n) = i1 {
                if n == ind {
                    g.disconnect_i1();
                } else if n > ind {
                    g.connect_i1(n - 1);
                }
            }
            if let Some(n) = i2 {
                if n == ind {
                    g.disconnect_i2();
                } else if n > ind {
                    g.connect_i2(n - 1);
                }
            }
        }
        self.gates.remove(ind);
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

    pub(crate) fn eval(&mut self) {
        for i in 0..self.gates.len() {
            match self[i] {
                And{i1, i2, output} => {
                    if i1.is_none() || i2.is_none() {
                        self[i] = And{i1, i2, output: None};
                    } else if self[i1.unwrap()].get_output().is_none() || self[i2.unwrap()]
                        .get_output().is_none() {
                        self[i] = And{i1, i2, output: None};
                    } else {
                        self[i] = And{i1, i2, output: Some(self[i1.unwrap()].get_output().unwrap()
                            && self[i2.unwrap()].get_output().unwrap())};
                    }
                },
                Or{i1, i2, output} => {
                    if i1.is_none() || i2.is_none() {
                        self[i] = Or{i1, i2, output: None};
                    } else if self[i1.unwrap()].get_output().is_none() || self[i2.unwrap()]
                        .get_output().is_none() {
                        self[i] = Or{i1, i2, output: None};
                    } else {
                        self[i] = Or{i1, i2, output: Some(self[i1.unwrap()].get_output().unwrap()
                            || self[i2.unwrap()].get_output().unwrap())};
                    }
                },
                Xor{i1, i2, output} => {
                    if i1.is_none() || i2.is_none() {
                        self[i] = Xor{i1, i2, output: None};
                    } else if self[i1.unwrap()].get_output().is_none() || self[i2.unwrap()]
                        .get_output().is_none() {
                        self[i] = Xor{i1, i2, output: None};
                    } else {
                        self[i] = Xor{i1, i2, output: Some(self[i1.unwrap()].get_output().unwrap()
                            != self[i2.unwrap()].get_output().unwrap())};
                    }
                },
                Not{i1, output} => {
                    if i1.is_none() {
                        self[i] = Not{i1, output: None};
                    } else if self[i1.unwrap()].get_output().is_none() {
                        self[i] = Not{i1, output: None};
                    } else {
                        self[i] = Not{i1, output: Some(!self[i1.unwrap()].get_output().unwrap())};
                    }
                },
                Nand{i1, i2, output} => {
                    if i1.is_none() || i2.is_none() {
                        self[i] = Nand{i1, i2, output: None};
                    } else if self[i1.unwrap()].get_output().is_none() || self[i2.unwrap()]
                        .get_output().is_none() {
                        self[i] = Nand{i1, i2, output: None};
                    } else {
                        self[i] = Nand{i1, i2, output: Some(!(self[i1.unwrap()].get_output().unwrap()
                            && self[i2.unwrap()].get_output().unwrap()))};
                    }
                },
                Nor{i1, i2, output} => {
                    if i1.is_none() || i2.is_none() {
                        self[i] = Nor{i1, i2, output: None};
                    } else if self[i1.unwrap()].get_output().is_none() || self[i2.unwrap()]
                        .get_output().is_none() {
                        self[i] = Nor{i1, i2, output: None};
                    } else {
                        self[i] = Nor{i1, i2, output: Some(!(self[i1.unwrap()].get_output().unwrap()
                            || self[i2.unwrap()].get_output().unwrap()))};
                    }
                },
                Xnor{i1, i2, output} => {
                    if i1.is_none() || i2.is_none() {
                        self[i] = Xnor{i1, i2, output: None};
                    } else if self[i1.unwrap()].get_output().is_none() || self[i2.unwrap()]
                        .get_output().is_none() {
                        self[i] = Xnor{i1, i2, output: None};
                    } else {
                        self[i] = Xnor{i1, i2, output: Some(self[i1.unwrap()].get_output().unwrap()
                            != self[i2.unwrap()].get_output().unwrap())};
                    }
                },
                _ => {}
            }
        }
    }
}