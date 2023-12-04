use crate::fmt::Printer;

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

impl Printer {
    pub fn rel_persistence(&mut self, n: &RelPersistence) {
        match n {
            RelPersistence::Temp => self.word("temporary "),
            RelPersistence::Unlogged => self.word("unlogged "),
            RelPersistence::Permanent => {}
            RelPersistence::Undefined => unreachable!(),
        }
    }
}
