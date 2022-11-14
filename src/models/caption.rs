use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum CaptionStyle {
    Round,
    Square,
}

impl std::fmt::Display for CaptionStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum CaptionColor {
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

impl std::fmt::Display for CaptionColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub const COLORS: [CaptionColor; 21] = [
	CaptionColor::Black,
	CaptionColor::White,
	CaptionColor::Red,
	CaptionColor::Pink,
	CaptionColor::Purple,
	CaptionColor::DeepPurple,
	CaptionColor::Indigo,
	CaptionColor::Blue,
	CaptionColor::LightBlue,
	CaptionColor::Cyan,
	CaptionColor::Teal,
	CaptionColor::Green,
	CaptionColor::LightGreen,
	CaptionColor::Lime,
	CaptionColor::Yellow,
	CaptionColor::Amber,
	CaptionColor::Orange,
	CaptionColor::DeepOrange,
	CaptionColor::Brown,
	CaptionColor::Grey,
	CaptionColor::BlueGrey];
