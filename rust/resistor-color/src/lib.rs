use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;
#[repr(u8)]
#[derive(Debug, PartialEq, IntoEnumIterator, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Blue = 1,
    Brown = 2,
    Green = 3,
    Grey = 4,
    Orange = 5,
    Red = 6,
    Violet = 7,
    White = 8,
    Yellow = 9,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    format!("{}", ResistorColor::from_int(value as u8))
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
