use crate::fmt;
use crate::partition_strategy::PartitionStrategy;
use pg_query::protobuf::PartitionBoundSpec;

impl fmt::Print for PartitionBoundSpec {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        if self.is_default {
            p.word("default");
            return Ok(());
        }

        p.word(" for values ");

        match self.strategy.clone().try_into().unwrap() {
            PartitionStrategy::Hash => {
                p.word(format!(
                    "with (modulus {}, remainder {})",
                    self.modulus, self.remainder
                ));
                Ok(())
            }
            PartitionStrategy::List => {
                p.word("in (");
                p.print_list(&self.listdatums);
                p.word(")");
                Ok(())
            }
            PartitionStrategy::Range => {
                p.word("from (");
                p.print_list(&self.lowerdatums);
                p.word(") to (");
                p.print_list(&self.upperdatums);
                p.word(")");
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
