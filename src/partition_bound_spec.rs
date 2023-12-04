use crate::fmt;
use crate::partition_strategy::PartitionStrategy;
use pg_query::protobuf::PartitionBoundSpec;

impl fmt::Print for PartitionBoundSpec {
    fn print(&self, p: &mut fmt::Printer) {
        if self.is_default {
            p.word("default");
            return;
        }

        p.word(" for values ");

        match self.strategy.clone().try_into().unwrap() {
            PartitionStrategy::Hash => {
                p.word(format!(
                    "with (modulus {}, remainder {})",
                    self.modulus, self.remainder
                ));
            }
            PartitionStrategy::List => {
                p.word("in (");
                p.print_list(&self.listdatums);
                p.word(")");
            }
            PartitionStrategy::Range => {
                p.word("from (");
                p.print_list(&self.lowerdatums);
                p.word(") to (");
                p.print_list(&self.upperdatums);
                p.word(")");
            }
            _ => unreachable!(),
        }
    }
}
