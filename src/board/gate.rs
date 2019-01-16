use board::circuit::Circuit;

pub trait Gate {
    fn get_input(&self, i: usize) -> usize;
    fn set_input(&mut self, i: usize, new_i: usize);
    fn num_inputs(&self) -> usize;
    fn get_output(&self, o: usize) -> usize;
    fn set_output(&mut self, o: usize, new_o: usize);
    fn num_outputs(&self) -> usize;
    fn eval(&self, circuit: &mut Circuit<Self>);
}