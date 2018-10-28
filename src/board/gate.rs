use std::fmt::{self, Debug};
use std::cell::Cell;
use std::rc::Rc;

// This is to implement the idea you were talking about, Leslie. You should be able to have a
// "counter" and if the counter's not up to snuff, then don't use it. Kinda like generational
// indices, but it's generational booleans. Then you just keep updating until all `Gate`s are
// have the correct 'num' value
#[derive(Copy, Clone, Debug)]
pub(crate) struct Evaluation {
    result: bool,
    num: usize
}

impl Evaluation {
    pub(crate) fn new(result: bool, num: usize) -> Self {
        Evaluation {
            result,
            num
        }
    }

    pub(crate) fn result(&self) -> bool {
        self.result
    }

    pub(crate) fn num(&self) -> usize {
        self.num
    }

    pub(crate) fn evaluated(&self, check: usize) -> bool {
        self.num == check
    }
}

#[derive(Clone)]
pub(crate) enum Gate {
    NoStorage {
        input: Box<[Option<Rc<Cell<Option<Evaluation>>>>]>,
        function: fn(&[Option<Rc<Cell<Option<Evaluation>>>>], &mut [Rc<Cell<Option<Evaluation>>>]),
        output: Box<[Rc<Cell<Option<Evaluation>>>]>
    },
    Storage {
        input: Box<[Option<Rc<Cell<Option<Evaluation>>>>]>,
        storage: Box<[bool]>,
        function: fn(&[Option<Rc<Cell<Option<Evaluation>>>>], &mut [bool],
            &mut [Rc<Cell<Option<Evaluation>>>]),
        output: Box<[Rc<Cell<Option<Evaluation>>>]>
    }
}

use Gate::*;

impl Debug for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {bb
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
        function: fn(&[Option<Rc<Cell<Option<Evaluation>>>>], &mut [Rc<Cell<Option<Evaluation>>>]))
        -> Self {
        NoStorage {
            input: vec![None; num_i].into_boxed_slice(),bbbbbbbbbbbbbb
            function,
            output: vec![Rc::new(Cell::new(None)); num_i].into_boxed_slice()
        }
    }

    pub(crate) fn new_s(num_i: usize, amt_storage: usize, num_o: usize,
        function: fn(&[Option<Rc<Cell<Option<Evaluation>>>>], &mut [bool],
            &mut [Rc<Cell<Option<Evaluation>>>])) -> Self {
        Storage {
            input: vec![None; num_i].into_boxed_slice(),
            storage: vec![false; amt_storage].into_boxed_slice(),
            function,
            output: vec![Rc::new(Cell::new(None)); num_i].into_boxed_slice() nn
        }
    }

    pub(crate) fn set_i_all(&mut self, new_i: Vec<Option<Rc<Cell<Option<Evaluation>>>>>) {
        match self {
            NoStorage{input, function: _, output: _} => *input = new_i.into_boxed_slice(),
            Storage{input, storage: _, function: _, output: _} => *input = new_i.into_boxed_slice()
        }
    }

    pub(crate) fn set_i_one(&mut self, i_ind: usize, new_i: Option<Rc<Cell<Option<Evaluation>>>>) {
        match self {
            NoStorage{input, function: _, output: _} => input[i_ind] = new_i,
            Storage{input, storage: _, function: _, output: _} => input[i_ind] = new_i
        }
    }

    pub(crate) fn get_input_one(&self, i_ind: usize) -> Option<Rc<Cell<Option<Evaluation>>>> {
        match self {
            NoStorage{input, function: _, output: _} => input[i_ind].clone(),
            Storage{input, storage: _, function: _, output: _} => input[i_ind].clone()
        }
    }

    pub(crate) fn get_inputs_all(&self) -> Vec<Option<Rc<Cell<Option<Evaluation>>>>> {
        match self {
            NoStorage{input, function: _, output: _} => {
                input.iter().map(|e| e.clone())
                    .collect::<Vec<Option<Rc<Cell<Option<Evaluation>>>>>>()
            },
            Storage{input, storage: _, function: _, output: _} => {
                input.iter().map(|e| e.clone())
                    .collect::<Vec<Option<Rc<Cell<Option<Evaluation>>>>>>()
            }
        }
    }

    pub(crate) fn get_outputs(&self) -> Vec<Rc<Cell<Option<Evaluation>>>> {
        match self {
            NoStorage{input: _, function: _, output} => {
                output.iter().map(|e| e.clone()).collect::<Vec<Rc<Cell<Option<Evaluation>>>>>()
            },
            Storage{input: _, storage: _, function: _, output} => {
                output.iter().map(|e| e.clone()).collect::<Vec<Rc<Cell<Option<Evaluation>>>>>()
            }
        }
    }

    pub(crate) fn get_output_one(&self, o_ind: usize) -> Rc<Cell<Option<Evaluation>>> {
        match self {
            NoStorage{input: _, function: _, output} => output[o_ind].clone(),
            Storage{input: _, storage: _, function: _, output} => output[o_ind].clone()
        }
    }

    pub(crate) fn set_ns_fn(&mut self, new_fn: fn(&[Option<Rc<Cell<Option<Evaluation>>>>],
        &mut [Rc<Cell<Option<Evaluation>>>])) {
        match self {
            NoStorage{input: _, function, output: _} => *function = new_fn,
            _ => {}
        }
    }

    pub(crate) fn set_s_fn(&mut self, new_fn: fn(&[Option<Rc<Cell<Option<Evaluation>>>>], &mut [bool],
            &mut [Rc<Cell<Option<Evaluation>>>])) {
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
        update_function: fn(&Box<[Option<Rc<Cell<Option<Evaluation>>>>]>,
        &mut Box<[Rc<Cell<Option<Evaluation>>>]>)) {
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