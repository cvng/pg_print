use crate::fmt;
use crate::utils::print_column_list;
use crate::utils::print_opt_with;
use pg_query::protobuf::ConstrType;
use pg_query::protobuf::Constraint;

impl fmt::Print for Constraint {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        if !self.conname.is_empty() {
            p.keyword("constraint ");
            p.ident(self.conname.clone());
            p.nbsp();
        }

        match self.contype() {
            ConstrType::ConstrDefault => {
                p.keyword("default ");
                self.raw_expr.as_deref()?.print(p)?;
            }
            ConstrType::ConstrPrimary => p.keyword("primary key"),
            ConstrType::ConstrUnique => p.keyword("unique"),
            _ => todo!("{:?}", self.contype()),
        }

        if !self.keys.is_empty() {
            p.nbsp();
            p.word("(");
            print_column_list(p, &self.keys);
            p.word(")");
        }

        match self.contype() {
            ConstrType::ConstrPrimary | ConstrType::ConstrUnique | ConstrType::ConstrExclusion => {
                print_opt_with(p, &self.options)?
            }
            _ => {}
        }

        if !self.indexspace.is_empty() {
            p.keyword("using index tablespace ");
            p.ident(self.indexspace.clone());
        }

        Some(())
    }
}
