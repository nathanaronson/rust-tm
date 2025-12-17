use crate::State;
use crate::Tape;
use crate::TapeError;
use crate::TransitionTable;

pub enum MachineStatus {
    Running,
    Halted,
}

pub struct TuringMachine {
    tape: Tape,
    state: State,
    transitions: TransitionTable,
}

impl TuringMachine {
    pub fn new(tape: Tape, state: State, transitions: TransitionTable) -> Self {
        TuringMachine {
            tape,
            state,
            transitions,
        }
    }

    pub fn step(&mut self) -> Result<MachineStatus, TapeError> {
        let current_symbol = self.tape.read();

        match self.transitions.get_rule(&self.state, &current_symbol) {
            Some((next_state, write_symbol, move_dir)) => {
                self.tape.write(write_symbol.clone());
                self.tape.move_head(move_dir.clone())?;
                self.state = next_state.clone();
                Ok(MachineStatus::Running)
            }
            None => Ok(MachineStatus::Halted),
        }
    }

    pub fn run(&mut self) {
        let mut steps = 0;
        println!("Initial State: {} | Tape: {}", self.state, self.tape);
        loop {
            match self.step() {
                Ok(MachineStatus::Running) => steps += 1,
                Ok(MachineStatus::Halted) => {
                    println!("Machine halted after {steps} steps.");
                    break;
                }
                Err(e) => {
                    println!("Error encountered at step {steps}: {e} ");
                    break;
                }
            }
        }

        println!("Final State: {} | Tape: {}", self.state, self.tape);
    }
}
