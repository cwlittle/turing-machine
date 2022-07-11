use turing_machine::*;
fn main() {
    Machine::new()
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
        .add_accept_state(4)
        .add_reject_state(5)
        .add_tape("0101")
        .run()
        .add_tape("1111101010000")
        .run()
        .add_tape("1011")
        .run();
}
