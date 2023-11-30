use crate::fmt;
use crate::fmt::gram::print_column_list;
use crate::fmt::gram::print_opt_with;
use pg_query::protobuf::ConstrType;
use pg_query::protobuf::Constraint;

impl fmt::Print for Constraint {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        if !self.conname.is_empty() {
            p.keyword("constraint ");
            p.ident(self.conname.clone());
            p.nbsp();
        }

        match self.contype() {
            ConstrType::ConstrDefault => {
                p.keyword("default ");
                if let Some(raw_expr) = &self.raw_expr {
                    raw_expr.print(p)?;
                }
            }
            ConstrType::ConstrPrimary => {
                p.keyword("primary key");
            }
            ConstrType::ConstrUnique => {
                p.keyword("unique");
            }
            ConstrType::ConstrCheck => {
                p.keyword("check (");
                if let Some(raw_expr) = &self.raw_expr {
                    raw_expr.print(p)?;
                }
                p.word(")");
            }
            ConstrType::ConstrNotnull => {
                p.keyword("not null");
            }
            _ => todo!("{:?}", self.contype()),
        }

        if !self.keys.is_empty() {
            p.nbsp();
            p.word("(");
            print_column_list(p, &self.keys)?;
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

        Ok(())
    }
}
