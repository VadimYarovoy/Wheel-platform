use color_eyre::Result;
use rppal::gpio::InputPin;

use super::engine::Direction;

pub struct ButtonInput {
    pub forward: InputPin,
    pub backward: InputPin,
    pub left: InputPin,
    pub right: InputPin,
}

impl ButtonInput {
    pub fn get_direction(&mut self) -> Direction {
        if self.forward.is_high() {
            return Direction::Forward;
        };
        if self.left.is_high() {
            return Direction::Left;
        };
        if self.right.is_high() {
            return Direction::Right;
        };
        if self.backward.is_high() {
            return Direction::Backward;
        };
        Direction::Stop
    }
}