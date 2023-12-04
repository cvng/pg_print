use crate::fmt;
use pg_query::protobuf::ConstrType;
use pg_query::protobuf::Constraint;

impl fmt::Print for Constraint {
    fn print(&self, p: &mut fmt::Printer) {
        if !self.conname.is_empty() {
            self.word("constraint ");
            self.ident(self.conname.clone());
            self.nbsp();
        }

        match self.contype() {
            ConstrType::ConstrDefault => {
                self.word("default ");
                if let Some(raw_expr) = &self.raw_expr {
                    self.node(raw_expr);
                }
            }
            ConstrType::ConstrPrimary => {
                self.word("primary key");
            }
            ConstrType::ConstrUnique => {
                self.word("unique");
            }
            ConstrType::ConstrCheck => {
                self.word("check (");
                if let Some(raw_expr) = &self.raw_expr {
                    self.node(raw_expr);
                }
                self.word(")");
            }
            ConstrType::ConstrNotnull => {
                self.word("not null");
            }
            _ => todo!("{:?}", self.contype()),
        }

        if !self.keys.is_empty() {
            self.nbsp();
            self.word("(");
            self.column_list(&self.keys);
            self.word(")");
        }

        match self.contype() {
            ConstrType::ConstrPrimary | ConstrType::ConstrUnique | ConstrType::ConstrExclusion => {
                self.opt_with(&self.options);
            }
            _ => {}
        }

        if !self.indexspace.is_empty() {
            self.word("using index tablespace ");
            self.ident(self.indexspace.clone());
        }
    }
}
