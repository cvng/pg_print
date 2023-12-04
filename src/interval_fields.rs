use crate::fmt;

const MONTH: i32 = 1;
const YEAR: i32 = 2;
const DAY: i32 = 3;
const HOUR: i32 = 10;
const MINUTE: i32 = 11;
const SECOND: i32 = 12;

pub const INTERVAL_FULL_RANGE: i32 = 0x7FFF;
pub const INTERVAL_FULL_PRECISION: i32 = 0xFFFF;

pub enum IntervalFields {
    Undefined,
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    YearToMonth,
    DayToHour,
    DayToMinute,
    DayToSecond,
    HourToMinute,
    HourToSecond,
    MinuteToSecond,
    FullRange,
}

impl From<i32> for IntervalFields {
    // See https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c#L3774.
    fn from(value: i32) -> Self {
        match value {
            x if x == 1 << YEAR => Self::Year,
            x if x == 1 << MONTH => Self::Month,
            x if x == 1 << DAY => Self::Day,
            x if x == 1 << HOUR => Self::Hour,
            x if x == 1 << MINUTE => Self::Minute,
            x if x == 1 << SECOND => Self::Second,
            x if x == 1 << YEAR | 1 << MONTH => Self::YearToMonth,
            x if x == 1 << DAY | 1 << HOUR => Self::DayToHour,
            x if x == 1 << DAY | 1 << HOUR | 1 << MINUTE => Self::DayToMinute,
            x if x == 1 << DAY | 1 << HOUR | 1 << MINUTE | 1 << SECOND => Self::DayToSecond,
            x if x == 1 << HOUR | 1 << MINUTE => Self::HourToMinute,
            x if x == 1 << HOUR | 1 << MINUTE | 1 << SECOND => Self::HourToSecond,
            x if x == 1 << MINUTE | 1 << SECOND => Self::MinuteToSecond,
            INTERVAL_FULL_RANGE => Self::FullRange,
            _ => Self::Undefined,
        }
    }
}

impl fmt::Print for IntervalFields {
    fn print(&self, p: &mut fmt::Printer) {
        match self {
            IntervalFields::Year => self.word(" year"),
            IntervalFields::Month => self.word(" month"),
            IntervalFields::Day => self.word(" day"),
            IntervalFields::Hour => self.word(" hour"),
            IntervalFields::Minute => self.word(" minute"),
            IntervalFields::Second => self.word(" second"),
            IntervalFields::YearToMonth => self.word(" year to month"),
            IntervalFields::DayToHour => self.word(" day to hour"),
            IntervalFields::DayToMinute => self.word(" day to minute"),
            IntervalFields::DayToSecond => self.word(" day to second"),
            IntervalFields::HourToMinute => self.word(" hour to minute"),
            IntervalFields::HourToSecond => self.word(" hour to second"),
            IntervalFields::MinuteToSecond => self.word(" minute to second"),
            IntervalFields::FullRange => {}
            IntervalFields::Undefined => unreachable!(),
        }
    }
}
