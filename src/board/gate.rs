use std::fmt::{self, Debug};
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) enum Gate {
    // This covers your basic gates like AND, OR, XOR, NOT, NAND, NOR, and XNOR. It also covers
    // other gates like MUXes and DMUXes. What gate 'NoStorage' is is determined by its function.
    // The number of inputs and outputs should be a fixed amount determined at runtime, hence the
    // use of 'Box<[T]>'.
    NoStorage {
        input: Box<[Option<Rc<Cell<Option<bool>>>>]>,
        function: fn(&[Option<Rc<Cell<Option<bool>>>>], &mut [Rc<Cell<Option<bool>>>]),
        output: Box<[Rc<Cell<Option<bool>>>]>
    },
    // This covers more complicated gates. What this can do is basically up to your imagination. RAM
    // modules of arbitrary size are possible, limited only by the amount of memory you're willing
    // to dedicate to the `Gate`. As for the use of 'Box<[T]>', see the end of the comment for the
    // 'NoStorage' variant.
    Storage {
        input: Box<[Option<Rc<Cell<Option<bool>>>>]>,
        storage: Box<[bool]>,
        function: fn(&[Option<Rc<Cell<Option<bool>>>>], &mut [bool],
            &mut [Rc<Cell<Option<bool>>>]),
        output: Box<[Rc<Cell<Option<bool>>>]>
    }
}

use Gate::*;

impl Debug for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NoStorage{input, function: _, output} => {
                write!(f, "i: {:?}, output: {:?}", input, output)
            },
            Storage{input, storage: _, function: _, output} => {
                write!(f, "i: {:?}, output: {:?}", input, output)
            }
        }
    }
}

impl Gate {
    pub(crate) fn new_ns(num_i: usize, num_o: usize,
        function: fn(&[Option<Rc<Cell<Option<bool>>>>], &mut [Rc<Cell<Option<bool>>>]))
        -> Self {
        NoStorage {
            input: vec![None; num_i].into_boxed_slice(),
            function,
            output: vec![Rc::new(Cell::new(None)); num_o].into_boxed_slice()
        }
    }

    pub(crate) fn new_s(num_i: usize, amt_storage: usize, num_o: usize,
        function: fn(&[Option<Rc<Cell<Option<bool>>>>], &mut [bool],
            &mut [Rc<Cell<Option<bool>>>])) -> Self {
        Storage {
            input: vec![None; num_i].into_boxed_slice(),
            storage: vec![false; amt_storage].into_boxed_slice(),
            function,
            output: vec![Rc::new(Cell::new(None)); num_o].into_boxed_slice()
        }
    }

    pub(crate) fn set_i(&mut self, i_ind: usize, new_i: Option<Rc<Cell<Option<bool>>>>) {
        match self {
            NoStorage{input, function: _, output: _} => input[i_ind] = new_i,
            Storage{input, storage: _, function: _, output: _} => input[i_ind] = new_i
        }
    }

    pub(crate) fn get_input(&self, i_ind: usize) -> Option<Rc<Cell<Option<bool>>>> {
        match self {
            NoStorage{input, function: _, output: _} => input[i_ind].clone(),
            Storage{input, storage: _, function: _, output: _} => input[i_ind].clone()
        }
    }

    pub(crate) fn get_inputs(&self) -> Vec<Option<Rc<Cell<Option<bool>>>>> {
        match self {
            NoStorage{input, function: _, output: _} => {
                input.iter().map(|e| e.clone())
                    .collect::<Vec<Option<Rc<Cell<Option<bool>>>>>>()
            },
            Storage{input, storage: _, function: _, output: _} => {
                input.iter().map(|e| e.clone())
                    .collect::<Vec<Option<Rc<Cell<Option<bool>>>>>>()
            }
        }
    }

    pub(crate) fn get_outputs(&self) -> Vec<Rc<Cell<Option<bool>>>> {
        match self {
            NoStorage{input: _, function: _, output} => {
                output.iter().map(|e| e.clone()).collect::<Vec<Rc<Cell<Option<bool>>>>>()
            },
            Storage{input: _, storage: _, function: _, output} => {
                output.iter().map(|e| e.clone()).collect::<Vec<Rc<Cell<Option<bool>>>>>()
            }
        }
    }

    pub(crate) fn get_outputs_options(&self) -> Box<[Rc<Cell<Option<bool>>>]> {
        match self {
            NoStorage{input: _, function: _, output} => output.clone(),
            Storage{input: _, storage: _, function: _, output} => output.clone()
        }
    }

    pub(crate) fn get_output(&self, o_ind: usize) -> Rc<Cell<Option<bool>>> {
        match self {
            NoStorage{input: _, function: _, output} => output[o_ind].clone(),
            Storage{input: _, storage: _, function: _, output} => output[o_ind].clone()
        }
    }

    pub(crate) fn get_output_value(&self, o_ind: usize) -> Option<bool> {
        match self {
            NoStorage{output, ..} => output[o_ind].get(),
            Storage{output, ..} => output[o_ind].get()
        }
    }

    pub(crate) fn set_ns_fn(&mut self, new_fn: fn(&[Option<Rc<Cell<Option<bool>>>>],
        &mut [Rc<Cell<Option<bool>>>])) {
        match self {
            NoStorage{input: _, function, output: _} => *function = new_fn,
            _ => {}
        }
    }

    pub(crate) fn set_s_fn(&mut self, new_fn: fn(&[Option<Rc<Cell<Option<bool>>>>],
        &mut [bool], &mut [Rc<Cell<Option<bool>>>])) {
        match self {
            Storage{input: _, storage: _, function, output: _} => *function = new_fn,
            _ => {}
        }
    }

    pub(crate) fn set_storage(&mut self, new_storage: Vec<bool>) {
        match self {
            Storage{input: _, storage, function: _, output: _} => {
                *storage = new_storage.into_boxed_slice();
            },
            _ => {}
        }
    }
    
    pub(crate) fn update_inputs(&mut self,
        update_function: fn(&[Option<Rc<Cell<Option<bool>>>>], &mut [Rc<Cell<Option<bool>>>])) {
        match self {
            NoStorage{input, function: _, output} => update_function(&*input, output),
            Storage{input, storage: _, function: _, output} => update_function(&*input, output)
        }
    }

    pub(crate) fn eval(&mut self) {
        match self {
            NoStorage{input, function, output} => function(&*input, output),
            Storage{input, storage, function, output} => function(&*input, storage, output),
        }
    }
}