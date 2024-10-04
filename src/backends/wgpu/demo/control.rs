use crate::input::InputAction;
use super::core::DemoApp;

pub fn handle_controls(
    inputs: &[InputAction],
    state: &mut DemoApp,
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
