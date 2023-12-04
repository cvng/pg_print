use crate::fmt::Printer;
use crate::partition_strategy::PartitionStrategy;
use pg_query::protobuf::PartitionBoundSpec;

impl Printer {
    pub fn partition_bound_spec(&self, n: &PartitionBoundSpec) {
        if n.is_default {
            self.word("default");
            return;
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
