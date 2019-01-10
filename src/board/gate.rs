use std::fmt::{self, Debug};
use std::cell::Cell;
use std::rc::Rc;

use board::lineregistry::LineRegistry;
use board::line::Line;

#[derive(Clone)]
pub enum Gate {
    NoStorage {
        input: Vec<Option<usize>>,
        function: fn(&[Option<usize>], &[bool], &mut LineRegistry),
        output: Vec<usize>
    },
    Storage {
        input: Vec<Option<usize>>,
        storage: Vec<bool>,
        function: fn(&[Option<usize>], &mut [bool], &[bool], &mut LineRegistry),
        output: Vec<usize>
    }
}

use self::Gate::*;

impl Debug for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NoStorage{input, output, ..} | Storage{input, output, ..} => {
                write!(f, "i: {:?}, output: {:?}", input, output)
            }
        }
    }
}

impl Gate {
    pub fn new_ns(num_i: usize, num_o: usize,
        function: fn(&[Option<usize>], &[bool], &mut LineRegistry), lr: &mut LineRegistry) -> Self {
        NoStorage {
            input: vec![None; num_i],
            function,
            output: vec![lr.add_line(); num_o]
        }
    }

    pub fn new_s(num_i: usize, amt_storage: usize, num_o: usize,
        function: fn(&[Option<usize>], &mut [bool], &[bool], &mut LineRegistry),
        lr: &mut LineRegistry) -> Self {
        Storage {
            input: vec![None; num_i],
            storage: vec![false; amt_storage],
            function,
            output: vec![lr.add_line(); num_o]
        }
    }

    pub fn set_i(&mut self, i_ind: usize, new_i: Option<usize>) {
        match self {
            NoStorage{input, ..} | Storage{input, ..} => input[i_ind] = new_i
        }
    }

    pub fn add_input(&mut self, new_i: Option<usize>) {
        match self {
            NoStorage{input, ..} | Storage{input, ..} => input.push(new_i)
        }
    }

    pub fn add_output(&mut self, lr: &mut LineRegistry) {
        match self {
            NoStorage{output, ..} | Storage{output, ..} => output.push(lr.add_line())
        }
    }

    pub fn get_input(&self, i_ind: usize) -> Option<usize> {
        match self {
            NoStorage{input, ..} | Storage{input, ..} => input[i_ind]
        }
    }

    pub fn get_inputs(&self) -> Vec<Option<usize>> {
        match self {
            NoStorage{input, ..} | Storage{input, ..} => input.clone()
        }
    }

    pub fn get_outputs(&self) -> Vec<usize> {
        match self {
            NoStorage{output, ..} | Storage {output, ..} => output.clone()
        }
    }

    pub fn get_output(&self, o_ind: usize) -> usize {
        match self {
            NoStorage{output, ..} | Storage{output, ..} => output[o_ind]
        }
    }

    pub fn set_ns_fn(&mut self, new_fn: fn(&[Option<usize>], &[bool], &mut LineRegistry)) {
        match self {
            NoStorage{function, ..} => *function = new_fn,
            _ => {}
        }
    }

    pub fn set_s_fn(&mut self,
        new_fn: fn(&[Option<usize>], &mut [bool], &[bool], &mut LineRegistry)) {
        match self {
            Storage{function, ..} => *function = new_fn,
            _ => {}
        }
    }

    pub fn set_storage(&mut self, new_storage: Vec<bool>) {
        match self {
            Storage{input: _, storage, function: _, output: _} => {
                *storage = new_storage.into_boxed_slice();
            },
            _ => {}
        }
    }

    pub fn update_inputs(&mut self,
        update_function: fn(&[Option<Rc<Cell<Line>>>], &mut [Rc<Cell<Line>>])) {
        match self {
            NoStorage{input, function: _, output} => update_function(&*input, output),
            Storage{input, storage: _, function: _, output} => update_function(&*input, output)
        }
    }

    pub fn eval(&mut self) {
        match self {
            NoStorage{input, function, output} => function(&*input, output),
            Storage{input, storage, function, output} => function(&*input, storage, output),
        }
    }
}