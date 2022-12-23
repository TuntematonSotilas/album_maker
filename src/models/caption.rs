#![allow(clippy::use_self)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum Style {
    Round,
    Square,
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum Color {
    Black,
    White,
    Red,
    Pink,
    Purple,
    DeepPurple,
    Indigo,
    Blue,
    LightBlue,
    Cyan,
    Teal,
    Green,
    LightGreen,
    Lime,
    Yellow,
    Amber,
    Orange,
    DeepOrange,
    Brown,
    Grey,
    BlueGrey,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub const COLORS: [Color; 21] = [
    Color::Black,
    Color::White,
    Color::Red,
    Color::Pink,
    Color::Purple,
    Color::DeepPurple,
    Color::Indigo,
    Color::Blue,
    Color::LightBlue,
    Color::Cyan,
    Color::Teal,
    Color::Green,
    Color::LightGreen,
    Color::Lime,
    Color::Yellow,
    Color::Amber,
    Color::Orange,
    Color::DeepOrange,
    Color::Brown,
    Color::Grey,
    Color::BlueGrey,
];
