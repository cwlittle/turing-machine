use std::collections::{HashMap, HashSet};
pub struct State<'a> {
    transition: &'a dyn Fn(&Option<char>, &mut Tape) -> u8,
}

impl<'a> State<'a> {
    pub fn new(transition: &'a dyn Fn(&Option<char>, &mut Tape) -> u8) -> Self {
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

struct Machine<'a> {
    counter: u8,
    alphabet: HashSet<Option<char>>,
    states: HashMap<u8, &'a State<'a>>,
    halt_accept: u8,
    halt_reject: u8,
    tape: Tape,
}

impl<'a> Machine<'a> {
    fn new(
        alphabet: HashSet<Option<char>>,
        states: HashMap<u8, &'a State<'a>>,
        halt_accept: u8,
        halt_reject: u8,
        tape: Tape,
    ) -> Self {
        Self {
            counter: 0,
            alphabet,
            states,
            halt_accept,
            halt_reject,
            tape,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_machine() {
        let state_0 = State::new(&|current_read: &Option<char>, tape: &mut Tape| -> u8 {
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
        });

        let state_1 = State::new(&|current_read: &Option<char>, tape: &mut Tape| -> u8 {
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
        });

        let state_2 = State::new(&|current_read: &Option<char>, tape: &mut Tape| -> u8 {
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
        });

        let state_3 = State::new(&|current_read: &Option<char>, tape: &mut Tape| -> u8 {
            match current_read {
                None => 4,
                _ => 5,
            }
        });

        let mut states = HashMap::new();
        states.insert(0, &state_0);
        states.insert(1, &state_1);
        states.insert(2, &state_2);
        states.insert(3, &state_3);

        let machine = Machine::new(
            HashSet::from_iter(alphabet.into_iter()),
            states,
            4,
            5,
            vec![Some('0'), Some('1'), Some('0'), Some('1')].into(),
        );

        machine.run();
    }
}
