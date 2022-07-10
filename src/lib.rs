use std::collections::{HashMap, HashSet};
struct State<'a> {
    index: u8,
    transition: &'a dyn Fn(&Option<char>, &mut u8) -> u8,
}

impl<'a> State<'a> {
    fn new(index: u8, transition: &'a dyn Fn(&Option<char>, &mut u8) -> u8) -> Self {
        State { index, transition }
    }
}

struct Tape {
    tape: Vec<Option<char>>,
}

impl From<Vec<Option<char>>> for Tape {
    fn from(tape: Vec<Option<char>>) -> Self {
        Tape { tape }
    }
}

impl Tape {
    fn new() -> Self {
        Self { tape: vec![None] }
    }

    fn write(&mut self, index: usize, value: Option<char>) {
        self.tape[index] = value;
    }

    fn read(&mut self, index: usize) -> Option<char> {
        self.tape.get(index).unwrap_or(&None).clone()
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

    fn run(mut self) {
        let mut next_state = 0;
        loop {
            let state = self.states.get(&next_state).unwrap();
            let current_read = self.tape.read(self.counter as usize);
            next_state = (state.transition)(&current_read, &mut self.counter);
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
        let alphabet = vec![Some('0'), Some('1')];
        let state_0 = State::new(0, &|current_read: &Option<char>, counter: &mut u8| -> u8 {
            match current_read {
                Some('0') => {
                    *counter += 1;
                    0
                }
                Some('1') => {
                    *counter += 1;
                    1
                }
                _ => 5,
            }
        });

        let state_1 = State::new(0, &|current_read: &Option<char>, counter: &mut u8| -> u8 {
            match current_read {
                Some('0') => {
                    *counter += 1;
                    2
                }
                Some('1') => {
                    *counter += 1;
                    1
                }
                _ => 5,
            }
        });

        let state_2 = State::new(0, &|current_read: &Option<char>, counter: &mut u8| -> u8 {
            match current_read {
                Some('0') => {
                    *counter += 1;
                    0
                }
                Some('1') => {
                    *counter += 1;
                    3
                }
                _ => 5,
            }
        });

        let state_3 = State::new(0, &|current_read: &Option<char>, counter: &mut u8| -> u8 {
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
