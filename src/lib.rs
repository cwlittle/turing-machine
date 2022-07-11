use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

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

impl Display for Tape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tape = self.tape.clone();
        let mut display_string = "".to_string();

        tape.iter().enumerate().for_each(|(i, c)| {
            if i == self.index {
                display_string += "┏━━━━━┓";
            } else {
                display_string += "┌─────┐";
            }
        });
        display_string += "\n";

        tape.iter().enumerate().for_each(|(i, c)| {
            let str = match c {
                Some(c) => c.to_string(),
                None => " ".to_string(),
            };
            if i == self.index {
                display_string += format!("┃  {}  ┃", str).as_str();
            } else {
                display_string += format!("│  {}  │", str).as_str();
            }
        });
        display_string += "\n";

        tape.iter().enumerate().for_each(|(i, c)| {
            if i == self.index {
                display_string += "┗━━━━━┛";
            } else {
                display_string += "└─────┘";
            }
        });

        write!(f, "{}", display_string)
    }
}

impl Debug for Tape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_string = "".to_string();
        for c in self.tape.iter() {
            match c {
                Some(c) => debug_string += &c.to_string(),
                None => debug_string += &" ".to_string(),
            }
        }

        write!(f, "{}", debug_string)
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

    pub fn run(mut self) -> Self {
        println!("Running machine with tape: {:?}", self.tape);
        let mut next_state = 0;
        loop {
            println!("Current state: {}", next_state);
            let state = self.states.get(&next_state).unwrap();
            let current_read = self.tape.read();
            println!("{}", self.tape);
            println!("");
            next_state = (state.transition)(&current_read, &mut self.tape);
            if next_state == self.halt_accept {
                println!("Accepted");
                println!("");
                return self;
            } else if next_state == self.halt_reject {
                println!("Rejected");
                println!("");
                return self;
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
