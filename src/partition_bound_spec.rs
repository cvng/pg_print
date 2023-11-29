use crate::fmt;
use crate::utils::print_expr_list;
use pg_query::protobuf::PartitionBoundSpec;

const PARTITION_STRATEGY_HASH: char = 'h';
const PARTITION_STRATEGY_LIST: char = 'l';
const PARTITION_STRATEGY_RANGE: char = 'r';

impl fmt::Print for PartitionBoundSpec {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        if self.is_default {
            p.keyword("default");
            return None;
        }

        p.keyword(" for values ");

        match self.strategy.chars().next().unwrap() {
            PARTITION_STRATEGY_HASH => {
                p.keyword(format!(
                    "with (modulus {}, remainder {})",
                    self.modulus, self.remainder
                ));
            }
            PARTITION_STRATEGY_LIST => {
                p.keyword("in (");
                print_expr_list(p, &self.listdatums);
                p.word(")");
            }
            PARTITION_STRATEGY_RANGE => {
                p.keyword("from (");
                print_expr_list(p, &self.lowerdatums);
                p.keyword(") to (");
                print_expr_list(p, &self.upperdatums);
                p.word(")");
            }
            _ => unreachable!(),
        };

        Some(())
    }
}
