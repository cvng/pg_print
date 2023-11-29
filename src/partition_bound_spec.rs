use crate::fmt;
use crate::partition_strategy::PartitionStrategy;
use crate::utils::print_expr_list;
use pg_query::protobuf::PartitionBoundSpec;

impl fmt::Print for PartitionBoundSpec {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        if self.is_default {
            p.keyword("default");
            return None;
        }

        p.keyword(" for values ");

        match self.strategy.clone().try_into().unwrap() {
            PartitionStrategy::Hash => {
                p.keyword(format!(
                    "with (modulus {}, remainder {})",
                    self.modulus, self.remainder
                ));
            }
            PartitionStrategy::List => {
                p.keyword("in (");
                print_expr_list(p, &self.listdatums);
                p.word(")");
            }
            PartitionStrategy::Range => {
                p.keyword("from (");
                print_expr_list(p, &self.lowerdatums);
                p.keyword(") to (");
                print_expr_list(p, &self.upperdatums);
                p.word(")");
            }
        };

        Some(())
    }
}
