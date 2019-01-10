use std::fmt::{self, Debug};
use std::cell::Cell;
use std::rc::Rc;

use board::line::Line;

#[derive(Clone)]
pub enum Gate {
    // This covers your basic gates like AND, OR, XOR, NOT, NAND, NOR, and XNOR. It also covers
    // other gates like MUXes and DMUXes. What gate 'NoStorage' is is determined by its function.
    // The number of inputs and outputs should be a fixed amount determined at runtime, hence the
    // use of 'Box<[T]>'.
    NoStorage {
        input: Vec<Rc<Cell<Line>>>,
        function: fn(&[Rc<Cell<Line>>], &mut [Rc<Cell<Line>>]),
        output: Vec<Rc<Cell<Line>>>
    },
    // This covers more complicated gates. What this can do is basically up to your imagination. RAM
    // modules of arbitrary size are possible, limited only by the amount of memory you're willing
    // to dedicate to the `Gate`. As for the use of 'Box<[T]>', see the end of the comment for the
    // 'NoStorage' variant.
    Storage {
        input: Vec<Rc<Cell<Line>>>,
        storage: Vec<Line>,
        function: fn(&[Rc<Cell<Line>>], &mut [Line], &mut [Rc<Cell<Line>>]),
        output: Vec<Rc<Cell<Line>>>
    }
}

use self::Gate::*;

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
    pub fn new_ns(num_i: usize, num_o: usize,
        function: fn(&[Rc<Cell<Line>>], &mut [Rc<Cell<Line>>])) -> Self {
        NoStorage {
            input: (0..num_i).map(|_| Rc::new(Cell::new(Line::Disconnected))).collect::<Vec<_>>(),
            function,
            output: (0..num_o).map(|_| Rc::new(Cell::new(Line::Low))).collect::<Vec<_>>()
        }
    }

    pub fn new_s(num_i: usize, amt_storage: usize, num_o: usize,
        function: fn(&[Rc<Cell<Line>>], &mut [Line], &mut [Rc<Cell<Line>>])) -> Self {
        Storage {
            input: (0..num_i).map(|_| Rc::new(Cell::new(Line::Disconnected))).collect::<Vec<_>>(),
            storage: vec![Line::Low; amt_storage],
            function,
            output: (0..num_o).map(|_| Rc::new(Cell::new(Line::Low))).collect::<Vec<_>>()
        }
    }

    pub fn set_input(&mut self, i_ind: usize, new_i: Rc<Cell<Line>>) {
        match self {
            NoStorage{input, function: _, output: _} => input[i_ind] = new_i,
            Storage{input, storage: _, function: _, output: _} => input[i_ind] = new_i
        }
    }

    pub fn get_input(&self, i_ind: usize) -> Rc<Cell<Line>> {
        match self {
            NoStorage{input, ..} | Storage{input, ..} => input[i_ind].clone()
        }
    }

    pub fn get_input_slice(&self) -> &[Rc<Cell<Line>>] {
        match self {
            NoStorage{input, ..} | Storage{input, ..} => &input
        }
    }

    pub fn get_input_vec(&self) -> Vec<Rc<Cell<Line>>> {
        match self {
            NoStorage{input, ..} | Storage{input, ..} => input.clone()
        }
    }

    pub fn get_output(&self, o_ind: usize) -> Rc<Cell<Line>> {
        match self {
            NoStorage{output, ..} | Storage{output, ..} => output[o_ind].clone(),
        }
    }

    pub fn get_output_slice(&self) -> &[Rc<Cell<Line>>] {
        match self {
            NoStorage{output, ..} | Storage{output, ..} => &output
        }
    }

    pub unsafe fn set_ns_fn(&mut self, new_fn: fn(&[Rc<Cell<Line>>], &mut [Rc<Cell<Line>>])) {
        match self {
            NoStorage{function, ..} => *function = new_fn,
            _ => panic!("Attempted to set a `NoStorage` instance's function to `Storage`'s type.")
        }
    }

    pub unsafe fn set_s_fn(&mut self,
        new_fn: fn(&[Rc<Cell<Line>>], &mut [Line], &mut [Rc<Cell<Line>>])) {
        match self {
            Storage{function, ..} => *function = new_fn,
            _ => panic!("Attempted to set a `Storage` instance's function to `NoStorage`'s type.")
        }
    }

    pub fn set_storage(&mut self, new_storage: Vec<Line>) {
        match self {
            Storage{ref mut storage, ..} => *storage = new_storage,
            _ => panic!("Tried to set the storage of a `Gate::NoStorage`.")
        }
    }

    pub fn reset_storage(&mut self) {
        match self {
            Storage{ref mut storage, ..} => {
                for line in storage {
                    *line = Line::Low;
                }
            },
            _ => panic!("Tried to reset the storage of a `Gate::NoStorage`.")
        }
    }

    pub fn splat_storage(&mut self, state: Line) {
        match self {
            Storage{ref mut storage, ..} => {
                for line in storage {
                    *line = state;
                }
            },
            _ => panic!("Tried to splat the storage of a `Gate::NoStorage`.")
        }
    }

    pub fn eval(&mut self) {
        match self {
            NoStorage{input, function, output} => function(input, output),
            Storage{input, storage, function, output} => function(input, storage, output),
        }
    }
}

impl Gate {
    pub fn high_source() -> Self {
        Gate::NoStorage {
            input: Vec::new(),
            function: |_, _| {},
            output: vec![Rc::new(Cell::new(Line::High)); 1]
        }
    }

    pub fn low_source() -> Self {
        Gate::NoStorage {
            input: Vec::new(),
            function: |_, _| {},
            output: vec![Rc::new(Cell::new(Line::Low)); 1]
        }
    }

    pub fn sink() -> Self {
        Gate::NoStorage {
            input: vec![Rc::new(Cell::new(Line::Disconnected))],
            function: |_, _| {},
            output: Vec::new()
        }
    }
}