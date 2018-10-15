#[derive(Copy, Clone, Debug)]
pub(crate) enum Gate {
    And {
        i1: Option<usize>,
        i2: Option<usize>,
        output: Option<bool>
    },
    Or {
        i1: Option<usize>,
        i2: Option<usize>,
        output: Option<bool>
    },
    Xor {
        i1: Option<usize>,
        i2: Option<usize>,
        output: Option<bool>
    },
    Not {
        i1: Option<usize>,
        output: Option<bool>
    },
    Nand {
        i1: Option<usize>,
        i2: Option<usize>,
        output: Option<bool>
    },
    Nor {
        i1: Option<usize>,
        i2: Option<usize>,
        output: Option<bool>
    },
    Xnor {
        i1: Option<usize>,
        i2: Option<usize>,
        output: Option<bool>
    },
    Source {
        output: bool
    },
}

use self::Gate::*;

impl Gate{
    // Checks to see if the input(s) are both Some(ref). If they are, the output is set to the
    // default value (false).
    pub(crate) fn update_inputs(&mut self) {
        match self {
            And{i1, i2, output} => {
                if i1.is_some() || i2.is_some() {
                    *output = Some(false);
                }
            },
            Or{i1, i2, output} => {
                if i1.is_some() || i2.is_some() {
                    *output = Some(false);
                }
            },
            Xor{i1, i2, output} => {
                if i1.is_some() || i2.is_some() {
                    *output = Some(false);
                }
            },
            Not{i1, output} => {
                if i1.is_some() {
                    *output = Some(false);
                }
            },
            Nand{i1, i2, output} => {
                if i1.is_some() || i2.is_some() {
                    *output = Some(false);
                }
            },
            Nor{i1, i2, output} => {
                if i1.is_some() || i2.is_some() {
                    *output = Some(false);
                }
            },
            Xnor{i1, i2, output} => {
                if i1.is_some() || i2.is_some() {
                    *output = Some(false);
                }
            },
            _ => {}
        }
    }

    pub(crate) fn get_output(&self) -> Option<bool> {
        match self {
            And{i1, i2, output} => *output,
            Or{i1, i2, output} => *output,
            Xor{i1, i2, output} => *output,
            Not{i1, output} => *output,
            Nand{i1, i2, output} => *output,
            Nor{i1, i2, output} => *output,
            Xnor{i1, i2, output} => *output,
            Source{output} => Some(*output)
        }
    }

    pub(crate) fn get_i1(&self) -> Option<usize> {
        match self {
            And{i1, i2, output} |
            Or{i1, i2, output} |
            Xor{i1, i2, output} |
            Nand{i1, i2, output} |
            Nor{i1, i2, output} |
            Xnor{i1, i2, output} => {
                *i1
            },
            Not{i1, output} => {
                *i1
            },
            _ => None
        }
    }

    pub(crate) fn get_i2(&self) -> Option<usize> {
        match self {
            And{i1, i2, output} |
            Or{i1, i2, output} |
            Xor{i1, i2, output} |
            Nand{i1, i2, output} |
            Nor{i1, i2, output} |
            Xnor{i1, i2, output} => {
                *i2
            },
            _ => None
        }
    }

    pub(crate) fn connect_i1(&mut self, t: usize) {
        match self {
            And{i1, i2, output} |
            Or{i1, i2, output} |
            Xor{i1, i2, output} |
            Nand{i1, i2, output} |
            Nor{i1, i2, output} |
            Xnor{i1, i2, output} => {
                *i1 = Some(t);
            },
            Not{i1, output} => {
                *i1 = Some(t);
            },
            _ => {}
        }
    }

    pub(crate) fn connect_i2(&mut self, t: usize) {
        match self {
            And{i1, i2, output} |
            Or{i1, i2, output} |
            Xor{i1, i2, output} |
            Nand{i1, i2, output} |
            Nor{i1, i2, output} |
            Xnor{i1, i2, output} => {
                *i2 = Some(t);
            },
            _ => {}
        }
    }

    pub(crate) fn disconnect_i1(&mut self) {
        match self {
            And{i1, i2, output} |
            Or{i1, i2, output} |
            Xor{i1, i2, output} |
            Nand{i1, i2, output} |
            Nor{i1, i2, output} |
            Xnor{i1, i2, output} => {
                *i1 = None;
            },
            Not{i1, output} => {
                *i1 = None;
            },
            _ => {}
        }
    }

    pub(crate) fn disconnect_i2(&mut self) {
        match self {
            And{i1, i2, output} |
            Or{i1, i2, output} |
            Xor{i1, i2, output} |
            Nand{i1, i2, output} |
            Nor{i1, i2, output} |
            Xnor{i1, i2, output} => {
                *i2 = None;
            },
            _ => {}
        }
    }

    pub(crate) fn set_high(&mut self) {
        match self {
            And{i1, i2, output} |
            Or{i1, i2, output} |
            Xor{i1, i2, output} |
            Nand{i1, i2, output} |
            Nor{i1, i2, output} |
            Xnor{i1, i2, output} => {},
            Not{i1, output} => {},
            Source{output} => *output = true
        }
    }

    pub(crate) fn set_low(&mut self) {
        match self {
            And{i1, i2, output} |
            Or{i1, i2, output} |
            Xor{i1, i2, output} |
            Nand{i1, i2, output} |
            Nor{i1, i2, output} |
            Xnor{i1, i2, output} => {},
            Not{i1, output} => {},
            Source{output} => *output = false
        }
    }

    pub(crate) fn new_and() -> Self {
        And{i1: None, i2: None, output: None}
    }

    pub(crate) fn new_or() -> Self {
        Or{i1: None, i2: None, output: None}
    }

    pub(crate) fn new_xor() -> Self {
        Xor{i1: None, i2: None, output: None}
    }

    pub(crate) fn new_not() -> Self {
        Not{i1: None, output: None}
    }

    pub(crate) fn new_nand() -> Self {
        Nand{i1: None, i2: None, output: None}
    }

    pub(crate) fn new_nor() -> Self {
        Nor{i1: None, i2: None, output: None}
    }

    pub(crate) fn new_xnor() -> Self {
        Xnor{i1: None, i2: None, output: None}
    }

    pub(crate) fn new_source() -> Self {
        Source{output: false}
    }
}