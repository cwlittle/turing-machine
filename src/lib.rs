use std::collections::HashMap;

pub struct State {
    transition: &'static dyn Fn(&Option<char>, &mut Tape) -> usize,
}

impl State {
    pub fn new(transition: &'static dyn Fn(&Option<char>, &mut Tape) -> usize) -> Self {
        State { transition }
    }
}

pub struct Tape {
    index: usize,
    tape: Vec<Option<char>>,
}

impl From<Vec<Option<char>>> for Tape {
    fn from(tape: Vec<Option<char>>) -> Self {
        Tape { index: 0, tape }
    }
}

impl Tape {
    pub fn new() -> Self {
        Self {
            index: 0,
            tape: vec![None],
        }
    }

    pub fn write(&mut self, value: Option<char>) {
        self.tape.insert(self.index, value);
    }

    pub fn read(&mut self) -> Option<char> {
        self.tape.get(self.index).unwrap_or(&None).clone()
    }

    pub fn move_left(&mut self) {
        self.index += 1;
        if self.index == self.tape.len() {
            self.tape.push(None);
        }
    }

    pub fn move_right(&mut self) {
        self.index -= 1;
        if self.index == self.tape.len() {
            self.tape.push(None);
        }
    }
}

impl From<&str> for Tape {
    fn from(s: &str) -> Self {
        let mut tape = Tape::new();
        for c in s.chars() {
            tape.write(Some(c));
            tape.move_left();
        }
        tape.index = 0;
        tape
    }
}
pub struct Machine {
    states: HashMap<usize, State>,
    halt_accept: usize,
    halt_reject: usize,
    tape: Tape,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            halt_accept: 0,
            halt_reject: 0,
            tape: Tape::new(),
        }
    }

    pub fn run(mut self) {
        let mut next_state = 0;
        loop {
            let state = self.states.get(&next_state).unwrap();
            let current_read = self.tape.read();
            next_state = (state.transition)(&current_read, &mut self.tape);
            if next_state == self.halt_accept {
                println!("Accepted");
                break;
            } else if next_state == self.halt_reject {
                println!("Rejected");
                break;
            }
        }
    }

    pub fn add_state(mut self, index: usize, state: State) -> Self {
        self.states.insert(index, state);
        self
    }

    pub fn add_accept_state(mut self, index: usize) -> Self {
        self.halt_accept = index;
        self
    }

    pub fn add_reject_state(mut self, index: usize) -> Self {
        self.halt_reject = index;
        self
    }

    pub fn add_tape(mut self, tape_string: &str) -> Self {
        self.tape = tape_string.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_machine() {
        let machine = Machine::new()
            .add_state(
                0,
                State::new(&|current_read: &Option<char>, tape: &mut Tape| -> usize {
                    match current_read {
                        Some('0') => {
                            tape.move_left();
                            0
                        }
                        Some('1') => {
                            tape.move_left();
                            1
                        }
                        _ => 5,
                    }
                }),
            )
            .add_state(
                1,
                State::new(&|current_read: &Option<char>, tape: &mut Tape| -> usize {
                    match current_read {
                        Some('0') => {
                            tape.move_left();
                            2
                        }
                        Some('1') => {
                            tape.move_left();
                            1
                        }
                        _ => 5,
                    }
                }),
            )
            .add_state(
                2,
                State::new(&|current_read: &Option<char>, tape: &mut Tape| -> usize {
                    match current_read {
                        Some('0') => {
                            tape.move_left();
                            0
                        }
                        Some('1') => {
                            tape.move_left();
                            3
                        }
                        _ => 5,
                    }
                }),
            )
            .add_state(
                3,
                State::new(&|current_read: &Option<char>, tape: &mut Tape| -> usize {
                    match current_read {
                        None => 4,
                        _ => 5,
                    }
                }),
            )
            .add_accept_state(4)
            .add_reject_state(5)
            .add_tape("0101")
            .run();
    }
}
