#![allow(unused)]

use statig::prelude::*;

#[derive(Default)]
pub struct Blinky;
pub struct Event;

#[state_machine(initial = "State::off()", state(derive(Debug)))]
impl Blinky {
    #[state]
    fn on(&mut self, event: &Event) -> Response<State> {
        Transition(State::off())
    }

    #[state]
    fn off(&mut self, event: &Event) -> Response<State> {
        Transition(State::on())
    }
}

fn main() {
    let state_machine = Blinky::default().state_machine();
    let mut state_machine = state_machine.init();
    state_machine.handle(&Event);
}

