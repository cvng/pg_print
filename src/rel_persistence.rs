use crate::fmt;

const RELPERSISTENCE_TEMP: char = 't';
const RELPERSISTENCE_UNLOGGED: char = 'u';
const RELPERSISTENCE_PERMANENT: char = 'p';

pub enum RelPersistence {
    Temp,
    Unlogged,
    Permanent,
}

impl fmt::Print for RelPersistence {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        match self {
            Self::Temp => Ok(p.keyword("temporary ")),
            Self::Unlogged => Ok(p.keyword("unlogged ")),
            Self::Permanent => Ok(()),
        }
    }
}

impl TryFrom<String> for RelPersistence {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.chars().next().unwrap() {
            RELPERSISTENCE_TEMP => Ok(Self::Temp),
            RELPERSISTENCE_UNLOGGED => Ok(Self::Unlogged),
            RELPERSISTENCE_PERMANENT => Ok(Self::Permanent),
            _ => Err(()),
        }
    }
}
