use crate::fmt;
use pg_query::protobuf::IndexElem;
use pg_query::protobuf::SortByDir;
use pg_query::protobuf::SortByNulls;

impl fmt::Print for IndexElem {
    fn print(&self, p: &mut fmt::Printer) {
        if !self.name.is_empty() {
            p.ident(self.name.clone());
        } else if let Some(expr) = &self.expr {
            p.node(expr);
        } else {
            unreachable!("{:?}", self);
        }

        p.opt_collate(&self.collation);

        if !self.opclass.is_empty() {
            p.any_name(&self.opclass);

            if !self.opclassopts.is_empty() {
                p.reloptions(&self.opclassopts);
            }

            p.nbsp();
        }

        match self.ordering() {
            SortByDir::SortbyAsc => p.word("asc "),
            SortByDir::SortbyDesc => p.word("desc "),
            _ => {}
        }

        match self.nulls_ordering() {
            SortByNulls::SortbyNullsFirst => p.word("nulls first "),
            SortByNulls::SortbyNullsLast => p.word("nulls last "),
            _ => {}
        }
    }
}
