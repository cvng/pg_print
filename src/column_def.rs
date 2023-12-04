use crate::fmt::Printer;
use pg_query::protobuf::ColumnDef;

impl Printer {
    pub fn column_def(&mut self, n: &ColumnDef) {
        if !n.colname.is_empty() {
            self.ident(n.colname.clone());
        }

        if let Some(type_name) = &n.type_name {
            self.nbsp();
            self.type_name(type_name);
        }

        if let Some(raw_default) = &n.raw_default {
            self.nbsp();
            self.word("using ");
            self.node(raw_default);
        }

        if !n.fdwoptions.is_empty() {
            self.nbsp();
            self.create_generic_options(&n.fdwoptions);
        }

        for constraint in n.constraints.iter() {
            self.nbsp();
            self.node(constraint);
        }

        if let Some(coll_clause) = &n.coll_clause {
            self.collate_clause(coll_clause);
        }
    }
}
