use board::gate::Gate::{self, Source, Or};

struct Sink {
    gates: Vec<Gate>,
    output: bool
}

impl Sink {
    fn new() -> Self {
        Sink {
            gates: Vec::new(),
            output: false
        }
    }

    fn add_input(&mut self) {
        
    }
}