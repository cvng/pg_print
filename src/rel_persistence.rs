use crate::fmt;

const RELPERSISTENCE_TEMP: char = 't';
const RELPERSISTENCE_UNLOGGED: char = 'u';
const RELPERSISTENCE_PERMANENT: char = 'p';

pub enum RelPersistence {
    Undefined,
    Temp,
    Unlogged,
    Permanent,
}

impl From<String> for RelPersistence {
    fn from(value: String) -> Self {
        match value.chars().next().unwrap() {
            RELPERSISTENCE_TEMP => Self::Temp,
            RELPERSISTENCE_UNLOGGED => Self::Unlogged,
            RELPERSISTENCE_PERMANENT => Self::Permanent,
            _ => Self::Undefined,
        }
    }
}

impl fmt::Print for RelPersistence {
    fn print(&self, p: &mut fmt::Printer) {
        match self {
            Self::Temp => p.word("temporary "),
            Self::Unlogged => p.word("unlogged "),
            Self::Permanent => {}
            Self::Undefined => unreachable!(),
        }
    }
}
