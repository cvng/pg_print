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

impl fmt::Print for IntervalFields {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        match self {
            IntervalFields::Year => p.word(" year"),
            IntervalFields::Month => p.word(" month"),
            IntervalFields::Day => p.word(" day"),
            IntervalFields::Hour => p.word(" hour"),
            IntervalFields::Minute => p.word(" minute"),
            IntervalFields::Second => p.word(" second"),
            IntervalFields::YearToMonth => p.word(" year to month"),
            IntervalFields::DayToHour => p.word(" day to hour"),
            IntervalFields::DayToMinute => p.word(" day to minute"),
            IntervalFields::DayToSecond => p.word(" day to second"),
            IntervalFields::HourToMinute => p.word(" hour to minute"),
            IntervalFields::HourToSecond => p.word(" hour to second"),
            IntervalFields::MinuteToSecond => p.word(" minute to second"),
            IntervalFields::FullRange => {}
            IntervalFields::Undefined => return Err(fmt::Error),
        }

        Ok(())
    }
}

impl TryFrom<i32> for IntervalFields {
    type Error = ();

    // See https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c#L3774.
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == 1 << YEAR => Ok(Self::Year),
            x if x == 1 << MONTH => Ok(Self::Month),
            x if x == 1 << DAY => Ok(Self::Day),
            x if x == 1 << HOUR => Ok(Self::Hour),
            x if x == 1 << MINUTE => Ok(Self::Minute),
            x if x == 1 << SECOND => Ok(Self::Second),
            x if x == 1 << YEAR | 1 << MONTH => Ok(Self::YearToMonth),
            x if x == 1 << DAY | 1 << HOUR => Ok(Self::DayToHour),
            x if x == 1 << DAY | 1 << HOUR | 1 << MINUTE => Ok(Self::DayToMinute),
            x if x == 1 << DAY | 1 << HOUR | 1 << MINUTE | 1 << SECOND => Ok(Self::DayToSecond),
            x if x == 1 << HOUR | 1 << MINUTE => Ok(Self::HourToMinute),
            x if x == 1 << HOUR | 1 << MINUTE | 1 << SECOND => Ok(Self::HourToSecond),
            x if x == 1 << MINUTE | 1 << SECOND => Ok(Self::MinuteToSecond),
            INTERVAL_FULL_RANGE => Ok(Self::FullRange),
            _ => Ok(Self::Undefined),
        }
    }
}
