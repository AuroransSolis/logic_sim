use std::ops::{Index, IndexMut};

use board::gate::Gate::{self, *};

#[derive(Debug)]
pub(crate) struct Circuit {
    pub(crate) gates: Vec<Gate>
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
        let ptr = self[tool_gate].get_output_ptr();
        self[target_gate].connect_i1(ptr);
    }

    pub(crate) fn connect_i1_ptr(&mut self, target_gate: usize,
        tool_gate_output_ptr: *const Option<bool>) {
        self[target_gate].connect_i1(tool_gate_output_ptr);
    }

    pub(crate) fn connect_i2(&mut self, target_gate: usize, tool_gate: usize) {
        let ptr = self[tool_gate].get_output_ptr();
        self[target_gate].connect_i2(ptr);
    }

    pub(crate) fn connect_i2_ptr(&mut self, target_gate: usize,
        tool_gate_output_ptr: *const Option<bool>) {
        self[target_gate].connect_i2(tool_gate_output_ptr);
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

    pub(crate) fn get_output_ptr(&self, g: usize) -> *const Option<bool> {
        self[g].get_output_ptr()
    }
    
    // Checks to see if the input(s) are both Some(ref). If they are, the output is set to the
    // default value (false).
    pub(crate) fn update_inputs(&mut self, g: usize) {
        match self[g] {
            And{i1, i2, output} => {
                if let (Some(i1_ptr), Some(i2_ptr)) = (i1, i2) {
                    unsafe {
                        if let (Some(_), Some(_)) = (*i1_ptr, *i2_ptr) {
                            self.eval(g);
                        } else {
                            self[g] = And{i1, i2, output: Some(false)};
                        }
                    }
                }
            },
            Or{i1, i2, output} => {
                if let (Some(i1_ptr), Some(i2_ptr)) = (i1, i2) {
                    unsafe {
                        if let (Some(_), Some(_)) = (*i1_ptr, *i2_ptr) {
                            self.eval(g);
                        }
                    }
                } else {
                    self[g] = Or{i1, i2, output: Some(false)};
                }
            },
            Xor{i1, i2, output} => {
                if let (Some(i1_ptr), Some(i2_ptr)) = (i1, i2) {
                    unsafe {
                        if let (Some(_), Some(_)) = (*i1_ptr, *i2_ptr) {
                            self.eval(g);
                        }
                    }
                } else {
                    self[g] = Xor{i1, i2, output: Some(false)};
                }
            },
            Nand{i1, i2, output} => {
                if let (Some(i1_ptr), Some(i2_ptr)) = (i1, i2) {
                    unsafe {
                        if let (Some(_), Some(_)) = (*i1_ptr, *i2_ptr) {
                            self.eval(g);
                        }
                    }
                } else {
                    self[g] = Nand{i1, i2, output: Some(false)};
                }
            },
            Nor{i1, i2, output} => {
                if let (Some(i1_ptr), Some(i2_ptr)) = (i1, i2) {
                    unsafe {
                        if let (Some(_), Some(_)) = (*i1_ptr, *i2_ptr) {
                            self.eval(g);
                        }
                    }
                } else {
                    self[g] = Nor{i1, i2, output: Some(false)};
                }
            },
            Xnor{i1, i2, output} => {
                if let (Some(i1_ptr), Some(i2_ptr)) = (i1, i2) {
                    unsafe {
                        if let (Some(_), Some(_)) = (*i1_ptr, *i2_ptr) {
                            self.eval(g);
                        }
                    }
                } else {
                    self[g] = Xnor{i1, i2, output: Some(false)};
                }
            },
            Not{i1, output} => {
                if let Some(i1_ptr) = i1 {
                    unsafe {
                        if let Some(_) = *i1_ptr {
                            self.eval(g);
                        }
                    }
                } else {
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
                if let (Some(i1_ptr), Some(i2_ptr)) = (i1, i2) {
                    unsafe {
                        if let (Some(b1), Some(b2)) = (*i1_ptr, *i2_ptr) {
                            self[g] = And{i1, i2, output: Some(b1 && b2)};
                        }
                    }
                } else {
                    self[g] = And{i1, i2, output: None};
                }
            },
            Or{i1, i2, output} => {
                if let (Some(i1_ptr), Some(i2_ptr)) = (i1, i2) {
                    unsafe {
                        if let (Some(b1), Some(b2)) = (*i1_ptr, *i2_ptr) {
                            self[g] = Or{i1, i2, output: Some(b1 || b2)};
                        }
                    }
                } else {
                    self[g] = Or{i1, i2, output: None};
                }
            },
            Xor{i1, i2, output} => {
                if let (Some(i1_ptr), Some(i2_ptr)) = (i1, i2) {
                    unsafe {
                        if let (Some(b1), Some(b2)) = (*i1_ptr, *i2_ptr) {
                            self[g] = Xor{i1, i2, output: Some(b1 != b2)};
                        }
                    }
                } else {
                    self[g] = Xor{i1, i2, output: None};
                }
            },
            Not{i1, output} => {
                if let Some(i1_ptr) = i1 {
                    unsafe {
                        if let Some(b1) = *i1_ptr {
                            self[g] = Not{i1, output: Some(!b1)};
                        }
                    }
                } else {
                    self[g] = Not{i1, output: None};
                }
            },
            Nand{i1, i2, output} => {
                if let (Some(i1_ptr), Some(i2_ptr)) = (i1, i2) {
                    unsafe {
                        if let (Some(b1), Some(b2)) = (*i1_ptr, *i2_ptr) {
                            self[g] = Nand{i1, i2, output: Some(!(b1 && b2))};
                        }
                    }
                } else {
                    self[g] = Nand{i1, i2, output: None};
                }
            },
            Nor{i1, i2, output} => {
                if let (Some(i1_ptr), Some(i2_ptr)) = (i1, i2) {
                    unsafe {
                        if let (Some(b1), Some(b2)) = (*i1_ptr, *i2_ptr) {
                            self[g] = Nor{i1, i2, output: Some(!(b1 || b2))};
                        }
                    }
                } else {
                    self[g] = Nor{i1, i2, output: None};
                }
            },
            Xnor{i1, i2, output} => {
                if let (Some(i1_ptr), Some(i2_ptr)) = (i1, i2) {
                    unsafe {
                        if let (Some(b1), Some(b2)) = (*i1_ptr, *i2_ptr) {
                            self[g] = Xnor{i1, i2, output: Some(b1 == b2)};
                        }
                    }
                } else {
                    self[g] = Xnor{i1, i2, output: None};
                }
            },
            _ => {}
        }
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
    // Otherwise, this function can easily leave dangling pointers in other 'Circuit's in the
    // 'Board'. The only time that this is safe is if the 'Circuit' has NO connections to other
    // 'Circuit's.
    pub(crate) unsafe fn remove_gate(&mut self, g: usize) {
        let ptr = self[g].get_output_ptr();
        for i in (0..self.gates.len()).filter(|&i| i != g) {
            if let Some(i1) = self[i].get_i1() {
                if i1 == ptr {
                    self[g].disconnect_i1();
                }
            }
            if let Some(i2) = self[i].get_i2() {
                if i2  == ptr {
                    self[g].disconnect_i2();
                }
            }
        }
        self.gates.remove(g);
    }
}