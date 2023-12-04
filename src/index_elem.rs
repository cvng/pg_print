use crate::fmt;
use pg_query::protobuf::IndexElem;
use pg_query::protobuf::SortByDir;
use pg_query::protobuf::SortByNulls;

impl fmt::Print for IndexElem {
    fn print(&self, p: &mut fmt::Printer) {
        if !self.name.is_empty() {
            self.ident(self.name.clone());
        } else if let Some(expr) = &self.expr {
            self.node(expr);
        } else {
            unreachable!("{:?}", self);
        }

        self.opt_collate(&self.collation);

        if !self.opclass.is_empty() {
            self.any_name(&self.opclass);

            if !self.opclassopts.is_empty() {
                self.reloptions(&self.opclassopts);
            }

            self.nbsp();
        }

        match self.ordering() {
            SortByDir::SortbyAsc => self.word("asc "),
            SortByDir::SortbyDesc => self.word("desc "),
            _ => {}
        }

        match self.nulls_ordering() {
            SortByNulls::SortbyNullsFirst => self.word("nulls first "),
            SortByNulls::SortbyNullsLast => self.word("nulls last "),
            _ => {}
        }
    }
}
