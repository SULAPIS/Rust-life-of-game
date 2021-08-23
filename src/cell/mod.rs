#[derive(Clone)]
pub struct Cell {
    state: bool,
}

impl Cell {
    pub fn new(state: bool) -> Cell {
        Cell { state }
    }

    pub fn is_alive(&self) -> bool {
        self.state
    }

    pub fn change_state(&mut self) {
        self.state = !self.state;
    }
}
