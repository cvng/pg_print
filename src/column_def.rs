use crate::fmt;
use crate::fmt::gram;
use pg_query::protobuf::ColumnDef;

impl fmt::Print for ColumnDef {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        if !self.colname.is_empty() {
            p.ident(self.colname.clone());
        }

        if let Some(type_name) = &self.type_name {
            p.nbsp();
            type_name.print(p)?;
        }

        if let Some(raw_default) = &self.raw_default {
            p.nbsp();
            p.word("using ");
            raw_default.print(p)?;
        }

        if !self.fdwoptions.is_empty() {
            p.nbsp();
            gram::create_generic_options(p, &self.fdwoptions)?;
        }

        for constraint in self.constraints.iter() {
            p.nbsp();
            constraint.print(p)?;
        }

        if let Some(coll_clause) = &self.coll_clause {
            coll_clause.print(p)?;
        }

        Ok(())
    }
}
