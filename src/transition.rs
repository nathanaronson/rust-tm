use crate::Direction;
use crate::State;
use crate::Symbol;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TransitionTable {
    rules: HashMap<(State, Symbol), (State, Symbol, Direction)>,
}

impl TransitionTable {
    pub fn new() -> Self {
        TransitionTable {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(
        &mut self,
        from_state: State,
        read_symbol: Symbol,
        to_state: State,
        write_symbol: Symbol,
        direction: Direction,
    ) {
        self.rules.insert(
            (from_state, read_symbol),
            (to_state, write_symbol, direction),
        );
    }

    pub fn get_rule(&self, state: &State, symbol: &Symbol) -> Option<&(State, Symbol, Direction)> {
        self.rules.get(&(state.clone(), symbol.clone()))
    }
}
