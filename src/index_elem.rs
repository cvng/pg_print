use crate::fmt;
use crate::fmt::gram;
use pg_query::protobuf::IndexElem;
use pg_query::protobuf::SortByDir;
use pg_query::protobuf::SortByNulls;

impl fmt::Print for IndexElem {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        if !self.name.is_empty() {
            p.ident(self.name.clone());
        } else if let Some(expr) = &self.expr {
            expr.print(p)?;
        } else {
            unreachable!("{:?}", self);
        }

        gram::print_opt_collate(p, &self.collation)?;

        if !self.opclass.is_empty() {
            gram::print_any_name(p, &self.opclass)?;

            if !self.opclassopts.is_empty() {
                gram::print_rel_options(p, &self.opclassopts)?;
            }

            p.nbsp();
        }

        match self.ordering() {
            SortByDir::SortbyAsc => p.word("asc "),
            SortByDir::SortbyDesc => p.word("desc "),
            _ => {}
        }

        match self.nulls_ordering() {
            SortByNulls::SortbyNullsFirst => p.keyword("nulls first "),
            SortByNulls::SortbyNullsLast => p.keyword("nulls last "),
            _ => {}
        }

        Ok(())
    }
}
