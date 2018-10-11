#[derive(Copy, Clone, Debug)]
pub(crate) enum Gate<'a> {
    And {
        i1: Option<&'a Gate<'a>>,
        i2: Option<&'a Gate<'a>>,
        output: Option<bool>
    },
    Or {
        i1: Option<&'a Gate<'a>>,
        i2: Option<&'a Gate<'a>>,
        output: Option<bool>
    },
    Xor {
        i1: Option<&'a Gate<'a>>,
        i2: Option<&'a Gate<'a>>,
        output: Option<bool>
    },
    Not {
        i1: Option<&'a Gate<'a>>,
        output: Option<bool>
    },
    Nand {
        i1: Option<&'a Gate<'a>>,
        i2: Option<&'a Gate<'a>>,
        output: Option<bool>
    },
    Nor {
        i1: Option<&'a Gate<'a>>,
        i2: Option<&'a Gate<'a>>,
        output: Option<bool>
    },
    Nxor {
        i1: Option<&'a Gate<'a>>,
        i2: Option<&'a Gate<'a>>,
        output: Option<bool>
    },
    Source {
        output: bool
    }
}

use self::Gate::*;

macro_rules! new_gate {
    (AND $i1:expr, $i2:expr) => {
        {
            let mut new = And{i1: $i1, i2: $i2, output: None};
            new.update_inputs();
            new
        }
    };
    (OR $i1:expr, $i2:expr) => {
        {
            let mut new = Or{i1: $i1, i2: $i2, output: None};
            new.update_inputs();
            new
        }
    };
    (XOR $i1:expr, $i2:expr) => {
        {
            let mut new = Xor{i1: $i1, i2: $i2, output: None};
            new.update_inputs();
            new
        }
    };
    (NOT $i1:expr) => {
        {
            let mut new = Not{i1: $i1, output: None};
            new.update_inputs();
            new
        }
    };
    (NAND $i1:expr, $i2:expr) => {
        {
            let mut new = Nand{i1: $i1, i2: $i2, output: None};
            new.update_inputs();
            new
        }
    };
    (NOR $i1:expr, $i2:expr) => {
        {
            let mut new = Nor{i1: $i1, i2: $i2, output: None};
            new.update_inputs();
            new
        }
    };
    (NXOR $i1:expr, $i2:expr) => {
        {
            let mut new = Nxor{i1: $i1, i2: $i2, output: None};
            new.update_inputs();
            new
        }
    };
    (SOURCE $val:expr) => {
        Source{output: $val}
    };
}

impl<'a> Gate<'a> {
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
            Nxor{i1, i2, output} => {
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
            Nxor{i1, i2, output} => *output,
            Source{output} => Some(*output)
        }
    }

    pub(crate) fn eval(self) -> Gate<'a> {
        match self {
            And{i1, i2, output} => {
                if i1.is_none() || i2.is_none() {
                    And{i1: i1, i2: i2, output: None}
                } else if i1.unwrap().get_output().is_none() || i2.unwrap().get_output().is_none() {
                    And{i1: i1, i2: i2, output: None}
                } else {
                    And{i1, i2, output: Some(i1.unwrap().get_output().unwrap() && i2.unwrap()
                            .get_output().unwrap())}
                }
            },
            Or{i1, i2, output} => {
                if i1.is_none() || i2.is_none() {
                    Or{i1: i1, i2: i2, output: None}
                } else if i1.unwrap().get_output().is_none() || i2.unwrap().get_output().is_none() {
                    Or{i1: i1, i2: i2, output: None}
                } else {
                    Or{i1, i2, output: Some(i1.unwrap().get_output().unwrap() || i2.unwrap()
                            .get_output().unwrap())}
                }
            },
            Xor{i1, i2, output} => {
                if i1.is_none() || i2.is_none() {
                    Xor{i1: i1, i2: i2, output: None}
                } else if i1.unwrap().get_output().is_none() || i2.unwrap().get_output().is_none() {
                    Xor{i1: i1, i2: i2, output: None}
                } else {
                    Xor{i1, i2, output: Some(i1.unwrap().get_output().unwrap() != i2.unwrap()
                            .get_output().unwrap())}
                }
            },
            Not{i1, output} => {
                if i1.is_none() {
                    Not{i1: i1, output: None}
                } else if i1.unwrap().get_output().is_none() {
                    Not{i1: i1, output: None}
                } else {
                    Not{i1, output: Some(!i1.unwrap().get_output().unwrap())}
                }
            },
            Nand{i1, i2, output} => {
                if i1.is_none() || i2.is_none() {
                    And{i1: i1, i2: i2, output: None}
                } else if i1.unwrap().get_output().is_none() || i2.unwrap().get_output().is_none() {
                    And{i1: i1, i2: i2, output: None}
                } else {
                    And{i1, i2, output: Some(!(i1.unwrap().get_output().unwrap() && i2.unwrap()
                            .get_output().unwrap()))}
                }
            },
            Nor{i1, i2, output} => {
                if i1.is_none() || i2.is_none() {
                    And{i1: i1, i2: i2, output: None}
                } else if i1.unwrap().get_output().is_none() || i2.unwrap().get_output().is_none() {
                    And{i1: i1, i2: i2, output: None}
                } else {
                    And{i1, i2, output: Some(!(i1.unwrap().get_output().unwrap() || i2.unwrap()
                            .get_output().unwrap()))}
                }
            },
            Nxor{i1, i2, output} => {
                if i1.is_none() || i2.is_none() {
                    Xor{i1: i1, i2: i2, output: None}
                } else if i1.unwrap().get_output().is_none() || i2.unwrap().get_output().is_none() {
                    Xor{i1: i1, i2: i2, output: None}
                } else {
                    Xor{i1, i2, output: Some(i1.unwrap().get_output().unwrap() == i2.unwrap()
                            .get_output().unwrap())}
                }
            },
            _ => self
        }
    }

    unsafe fn set_output(&self, v: Option<bool>) {
        match self {
            And{i1, i2, output} => {
                let output_ptr = output as *const Option<bool>;
                let output_ptr = output_ptr as *mut Option<bool>;
                *output_ptr = v;
            },
            Or{i1, i2, output} => {
                let output_ptr = output as *const Option<bool>;
                let output_ptr = output_ptr as *mut Option<bool>;
                *output_ptr = v;
            },
            Xor{i1, i2, output} => {
                let output_ptr = output as *const Option<bool>;
                let output_ptr = output_ptr as *mut Option<bool>;
                *output_ptr = v;
            },
            Not{i1, output} => {
                let output_ptr = output as *const Option<bool>;
                let output_ptr = output_ptr as *mut Option<bool>;
                *output_ptr = v;
            },
            Nand{i1, i2, output} => {
                let output_ptr = output as *const Option<bool>;
                let output_ptr = output_ptr as *mut Option<bool>;
                *output_ptr = v;
            },
            Nor{i1, i2, output} => {
                let output_ptr = output as *const Option<bool>;
                let output_ptr = output_ptr as *mut Option<bool>;
                *output_ptr = v;
            },
            Nxor{i1, i2, output} => {
                let output_ptr = output as *const Option<bool>;
                let output_ptr = output_ptr as *mut Option<bool>;
                *output_ptr = v;
            },
            _ => {}
        }
    }
}