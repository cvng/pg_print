const PARTITION_STRATEGY_HASH: char = 'h';
const PARTITION_STRATEGY_LIST: char = 'l';
const PARTITION_STRATEGY_RANGE: char = 'r';

pub enum PartitionStrategy {
    Hash,
    List,
    Range,
}

impl TryFrom<String> for PartitionStrategy {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.chars().next().unwrap() {
            PARTITION_STRATEGY_HASH => Ok(Self::Hash),
            PARTITION_STRATEGY_LIST => Ok(Self::List),
            PARTITION_STRATEGY_RANGE => Ok(Self::Range),
            _ => Err(()),
        }
    }
}
