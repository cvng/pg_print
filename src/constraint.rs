use crate::fmt;
use pg_query::protobuf::ConstrType;
use pg_query::protobuf::Constraint;

impl fmt::Print for Constraint {
    fn print(&self, p: &mut fmt::Printer) {
        if !self.conname.is_empty() {
            p.word("constraint ");
            p.ident(self.conname.clone());
            p.nbsp();
        }

        match self.contype() {
            ConstrType::ConstrDefault => {
                p.word("default ");
                if let Some(raw_expr) = &self.raw_expr {
                    p.node(raw_expr);
                }
            }
            ConstrType::ConstrPrimary => {
                p.word("primary key");
            }
            ConstrType::ConstrUnique => {
                p.word("unique");
            }
            ConstrType::ConstrCheck => {
                p.word("check (");
                if let Some(raw_expr) = &self.raw_expr {
                    p.node(raw_expr);
                }
                p.word(")");
            }
            ConstrType::ConstrNotnull => {
                p.word("not null");
            }
            _ => todo!("{:?}", self.contype()),
        }

        if !self.keys.is_empty() {
            p.nbsp();
            p.word("(");
            p.column_list(&self.keys);
            p.word(")");
        }

        match self.contype() {
            ConstrType::ConstrPrimary | ConstrType::ConstrUnique | ConstrType::ConstrExclusion => {
                p.opt_with(&self.options);
            }
            _ => {}
        }

        if !self.indexspace.is_empty() {
            p.word("using index tablespace ");
            p.ident(self.indexspace.clone());
        }
    }
}
