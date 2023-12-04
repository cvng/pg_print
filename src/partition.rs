use crate::fmt::Printer;
use pg_query::protobuf::PartitionBoundSpec;

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

impl Printer {
    pub fn partition_bound_spec(&mut self, n: &PartitionBoundSpec) {
        if n.is_default {
            return self.word("default");
        }

        self.word(" for values ");

        match n.strategy.clone().try_into().unwrap() {
            PartitionStrategy::Hash => {
                self.word(format!(
                    "with (modulus {}, remainder {})",
                    n.modulus, n.remainder
                ));
            }
            PartitionStrategy::List => {
                self.word("in (");
                self.print_list(&n.listdatums);
                self.word(")");
            }
            PartitionStrategy::Range => {
                self.word("from (");
                self.print_list(&n.lowerdatums);
                self.word(") to (");
                self.print_list(&n.upperdatums);
                self.word(")");
            }
            _ => unreachable!(),
        }
    }
}
