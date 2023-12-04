use crate::fmt;
use pg_query::protobuf::ColumnDef;

impl fmt::Print for ColumnDef {
    fn print(&self, p: &mut fmt::Printer) {
        if !self.colname.is_empty() {
            p.ident(self.colname.clone());
        }

        if let Some(type_name) = &self.type_name {
            p.nbsp();
            type_name.print(p);
        }

        if let Some(raw_default) = &self.raw_default {
            p.nbsp();
            p.word("using ");
            p.node(raw_default);
        }

        if !self.fdwoptions.is_empty() {
            p.nbsp();
            p.create_generic_options(&self.fdwoptions);
        }

        for constraint in self.constraints.iter() {
            p.nbsp();
            p.node(constraint);
        }

        if let Some(coll_clause) = &self.coll_clause {
            coll_clause.print(p);
        }
    }
}
