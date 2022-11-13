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
	DeepPruple,
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