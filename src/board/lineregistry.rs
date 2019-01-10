use board::line::Line;

#[derive(Clone)]
pub struct LineRegistry {
    lines: Vec<Line>
}

impl LineRegistry {
    pub fn drive_high(&mut self, line: usize) {
        self[line] = Line::High;
    }

    pub fn drive_low(&mut self, line: usize) {
        self[line] = Line::Low;
    }

    pub fn set_line_state(&mut self, line: usize, state: Line) {
        self[line] = state;
    }

    pub fn get_line_state(&self, line: usize) -> Line {
        self[line]
    }

    pub fn remove(&mut self, line: usize) {
        let _ = self.lines.remove(line);
    }

    pub fn add_line(&mut self) -> usize {
        self.lines.push(Line::Low);
        self.lines.len() - 1
    }
}