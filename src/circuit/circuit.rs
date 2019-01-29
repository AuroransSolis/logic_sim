use circuit::gate::Gate;
use circuit::line::Line;

pub struct Circuit {
    pub(crate) gates: Vec<Box<dyn Gate>>,
    pub(crate) lines: Vec<Line>,
    pub(crate) inputs: Vec<usize>,
    pub(crate) outputs: Vec<usize>
}

impl Circuit {
    pub(crate) fn new() -> Self {
        Circuit {
            gates: Vec::new(),
            lines: vec![Line::Disconnected],
            inputs: Vec::new(),
            outputs: Vec::new()
        }
    }

    pub(crate) fn add_gate<T: Gate + 'static>(&mut self, mut gate: T) -> usize {
        for i in 0..gate.num_inputs() {
            gate.set_input(i, 0);
        }
        for i in 0..gate.num_outputs() {
            self.lines.push(Line::Low);
            gate.set_output(i, self.lines.len() - 1);
        }
        self.gates.push(Box::new(gate));
        self.gates.len() - 1
    }

    pub(crate) fn add_line(&mut self, line: Line) -> usize {
        self.lines.push(line);
        self.lines.len() - 1
    }

    pub(crate) fn get_gate_input(&self, target_gate: usize, target_gate_input: usize) -> usize {
        self.gates[target_gate].get_input(target_gate_input)
    }

    pub(crate) fn get_gate_output(&self, target_gate: usize, target_gate_output: usize) -> usize{
        self.gates[target_gate].get_output(target_gate_output)
    }

    pub(crate) fn set_line(&mut self, line: usize, state: Line) {
        self.lines[line] = state;
    }

    pub(crate) fn set_gate_input(&mut self, target_gate: usize, target_gate_input: usize,
        line: usize) {
        self.gates[target_gate].set_input(target_gate_input, line);
    }

    pub(crate) fn get_line_state(&self, target_line: usize) -> Line {
        self.lines[target_line]
    }

    pub(crate) fn connect_i_single(&mut self, target_gate: usize, target_gate_input: usize,
        tool_gate: usize, tool_gate_output: usize) {
        let new_input = self.gates[tool_gate].get_output(tool_gate_output);
        self.gates[target_gate].set_input(target_gate_input, new_input);
    }

    pub(crate) fn disconnect_i_single(&mut self, target_gate: usize, target_gate_input: usize) {
        self.gates[target_gate].set_input(target_gate_input, 0);
    }

    pub(crate) fn eval_single_gate(&mut self, g: usize) {
        self.gates[g].eval(&mut self.lines);
    }

    pub(crate) fn eval(&mut self) {
        for gate in &mut self.gates {
            gate.eval(&mut self.lines)
        }
    }

    pub(crate) fn eval_n_passes(&mut self, passes: usize) {
        for _ in 0..passes {
            self.eval();
        }
    }

    pub(crate) fn eval_n_evals_per_gate(&mut self, evals: usize) {
        for gate in &mut self.gates {
            for _ in 0..evals {
                gate.eval(&mut self.lines);
            }
        }
    }

    pub(crate) fn remove_gate(&mut self, gate: usize) {
        let outputs = (0..self.gates[gate].num_outputs()).map(|o| self.gates[gate].get_output(o))
            .collect::<Vec<_>>();
        if self.gates[gate].num_inputs() == 0 {
            for i in (0..self.outputs.len()).rev() {
                if outputs.contains(&self.outputs[i]) {
                    self.outputs.remove(i);
                }
            }
        }
        self.gates.remove(gate);
        for g in &mut self.gates {
            for i in 0..g.num_inputs() {
                if outputs.contains(&g.get_input(i)) {
                    g.set_input(i, 0);
                } else {
                    let ind = g.get_input(i);
                    let shift = outputs.iter().take_while(|&&output_ind| output_ind < ind).count();
                    g.set_input(i, ind - shift);
                }
            }
            for o in 0..g.num_outputs() {
                let ind = g.get_output(o);
                let shift = outputs.iter().take_while(|&&output_ind| output_ind < ind).count();
                g.set_output(o, ind - shift);
            }
        }
        for output_ind in outputs.into_iter().rev() {
            self.lines.remove(output_ind);
        }
    }
}