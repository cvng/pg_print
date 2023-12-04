use crate::fmt::Printer;
use pg_query::protobuf::IndexElem;
use pg_query::protobuf::SortByDir;
use pg_query::protobuf::SortByNulls;

impl Printer {
    pub fn index_elem(&self, n: &IndexElem) {
        if !n.name.is_empty() {
            self.ident(n.name.clone());
        } else if let Some(expr) = &n.expr {
            self.node(expr);
        } else {
            unreachable!();
        }

        self.opt_collate(&n.collation);

        if !n.opclass.is_empty() {
            self.any_name(&n.opclass);

            if !n.opclassopts.is_empty() {
                self.reloptions(&n.opclassopts);
            }

            self.nbsp();
        }

        match n.ordering() {
            SortByDir::SortbyAsc => self.word("asc "),
            SortByDir::SortbyDesc => self.word("desc "),
            _ => {}
        }

        match n.nulls_ordering() {
            SortByNulls::SortbyNullsFirst => self.word("nulls first "),
            SortByNulls::SortbyNullsLast => self.word("nulls last "),
            _ => {}
        }
    }
}
