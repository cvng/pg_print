use crate::fmt::Printer;
use pg_query::protobuf::ConstrType;
use pg_query::protobuf::Constraint;

impl Printer {
    pub fn constraint(&mut self, n: &Constraint) {
        if !n.conname.is_empty() {
            self.word("constraint ");
            self.ident(n.conname.clone());
            self.nbsp();
        }

        match n.contype() {
            ConstrType::ConstrDefault => {
                self.word("default ");
                if let Some(raw_expr) = &n.raw_expr {
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
                if let Some(raw_expr) = &n.raw_expr {
                    self.node(raw_expr);
                }
                self.word(")");
            }
            ConstrType::ConstrNotnull => {
                self.word("not null");
            }
            _ => todo!(),
        }

        if !n.keys.is_empty() {
            self.nbsp();
            self.word("(");
            self.column_list(&n.keys);
            self.word(")");
        }

        match n.contype() {
            ConstrType::ConstrPrimary | ConstrType::ConstrUnique | ConstrType::ConstrExclusion => {
                self.opt_with(&n.options);
            }
            _ => {}
        }

        if !n.indexspace.is_empty() {
            self.word("using index tablespace ");
            self.ident(n.indexspace.clone());
        }
    }

    pub fn opt_no_inherit(&mut self, no_inherit: bool) {
        if no_inherit {
            self.word("no inherit ");
        }
    }
}
