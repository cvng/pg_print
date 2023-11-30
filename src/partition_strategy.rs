const PARTITION_STRATEGY_HASH: char = 'h';
const PARTITION_STRATEGY_LIST: char = 'l';
const PARTITION_STRATEGY_RANGE: char = 'r';

pub enum PartitionStrategy {
    Undefined,
    Hash,
    List,
    Range,
}

impl From<String> for PartitionStrategy {
    fn from(value: String) -> Self {
        match value.chars().next().unwrap() {
            PARTITION_STRATEGY_HASH => Self::Hash,
            PARTITION_STRATEGY_LIST => Self::List,
            PARTITION_STRATEGY_RANGE => Self::Range,
            _ => Self::Undefined,
        }
    }
}
