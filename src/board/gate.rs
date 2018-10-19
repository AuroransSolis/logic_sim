#[derive(Copy, Clone, Debug)]
pub(crate) enum Gate {
    And {
        i1: Option<*const Option<bool>>,
        i2: Option<*const Option<bool>>,
        output: Option<bool>
    },
    Or {
        i1: Option<*const Option<bool>>,
        i2: Option<*const Option<bool>>,
        output: Option<bool>
    },
    Xor {
        i1: Option<*const Option<bool>>,
        i2: Option<*const Option<bool>>,
        output: Option<bool>
    },
    Not {
        i1: Option<*const Option<bool>>,
        output: Option<bool>
    },
    Nand {
        i1: Option<*const Option<bool>>,
        i2: Option<*const Option<bool>>,
        output: Option<bool>
    },
    Nor {
        i1: Option<*const Option<bool>>,
        i2: Option<*const Option<bool>>,
        output: Option<bool>
    },
    Xnor {
        i1: Option<*const Option<bool>>,
        i2: Option<*const Option<bool>>,
        output: Option<bool>
    },
    Source {
        output: Option<bool>
    },
}

use self::Gate::*;

impl Gate {
    pub(crate) fn get_output(&self) -> Option<bool> {
        match self {
            And{i1, i2, output} |
            Or{i1, i2, output} |
            Xor{i1, i2, output} |
            Nand{i1, i2, output} |
            Nor{i1, i2, output} |
            Xnor{i1, i2, output} => *output,
            Not{i1, output} => *output,
            Source{output} => *output
        }
    }

    pub(crate) fn get_output_ptr(&self) -> *const Option<bool> {
        match self {
            And{i1, i2, output} |
            Or{i1, i2, output} |
            Xor{i1, i2, output} |
            Nand{i1, i2, output} |
            Nor{i1, i2, output} |
            Xnor{i1, i2, output} => output,
            Not{i1, output} => output,
            Source{output} => output
        }
    }

    pub(crate) fn get_i1(&self) -> Option<*const Option<bool>> {
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

    pub(crate) fn get_i2(&self) -> Option<*const Option<bool>> {
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

    pub(crate) fn connect_i1(&mut self, t: *const Option<bool>) {
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

    pub(crate) fn connect_i2(&mut self, t: *const Option<bool>) {
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
            Source{output} => *output = Some(true)
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
            Source{output} => *output = Some(false)
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
        Source{output: Some(false)}
    }
}