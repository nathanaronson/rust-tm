use rust_tm::{Direction, Symbol, Tape, TransitionTable, TuringMachine};

/// Example Use of Turing Machine Library
/// Turing Machine for Palindrome Verification (Class P)
fn main() {
    let mut transitions = TransitionTable::new();

    let start = "start".to_string();
    let look0 = "look0".to_string();
    let look1 = "look1".to_string();
    let check0 = "check0".to_string();
    let check1 = "check1".to_string();
    let back = "back".to_string();
    let accept = "accept".to_string();
    let reject = "reject".to_string();

    transitions.add_rule(start.clone(), Symbol::Zero, look0.clone(), Symbol::Blank, Direction::Right);
    transitions.add_rule(start.clone(), Symbol::One, look1.clone(), Symbol::Blank, Direction::Right);
    transitions.add_rule(start.clone(), Symbol::Blank, accept.clone(), Symbol::Blank, Direction::Right);

    for s in [look0.clone(), look1.clone()] {
        transitions.add_rule(s.clone(), Symbol::Zero, s.clone(), Symbol::Zero, Direction::Right);
        transitions.add_rule(s.clone(), Symbol::One, s.clone(), Symbol::One, Direction::Right);
    }

    transitions.add_rule(look0.clone(), Symbol::Blank, check0.clone(), Symbol::Blank, Direction::Left);
    transitions.add_rule(look1.clone(), Symbol::Blank, check1.clone(), Symbol::Blank, Direction::Left);

    transitions.add_rule(check0.clone(), Symbol::Zero, back.clone(), Symbol::Blank, Direction::Left);
    transitions.add_rule(check0.clone(), Symbol::Blank, accept.clone(), Symbol::Blank, Direction::Right);
    transitions.add_rule(check0.clone(), Symbol::One, reject.clone(), Symbol::One, Direction::Right);

    transitions.add_rule(check1.clone(), Symbol::One, back.clone(), Symbol::Blank, Direction::Left);
    transitions.add_rule(check1.clone(), Symbol::Blank, accept.clone(), Symbol::Blank, Direction::Right);
    transitions.add_rule(check1.clone(), Symbol::Zero, reject.clone(), Symbol::Zero, Direction::Right);

    transitions.add_rule(back.clone(), Symbol::Zero, back.clone(), Symbol::Zero, Direction::Left);
    transitions.add_rule(back.clone(), Symbol::One, back.clone(), Symbol::One, Direction::Left);
    transitions.add_rule(back.clone(), Symbol::Blank, start.clone(), Symbol::Blank, Direction::Right);

    let input: Vec<Symbol> = vec![Symbol::One, Symbol::Zero, Symbol::One, Symbol::One, Symbol::Zero, Symbol::One];
    let tape = Tape::new(input);

    let mut machine = TuringMachine::new(tape, start, transitions);
    
    machine.run();
}