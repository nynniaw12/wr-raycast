use crate::input::InputAction;
use super::state::DemoGameState;

pub fn handle_controls(
    inputs: &[InputAction],
    state: &mut DemoGameState,
) {
    for action in inputs {
        match action {
            InputAction::C => state.c(),
            InputAction::T => state.t(),
            InputAction::S => state.s(),
            InputAction::MoveForward => state.up(),
            InputAction::MoveBackward => state.down(),
            _ => ()
        }
    }
}
