use crate::fmt;
use pg_query::protobuf::ColumnDef;
use pg_query::NodeEnum;
use pg_query::Node;

impl fmt::Print for ColumnDef {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        if !self.colname.is_empty() {
            p.ident(self.colname.clone());
        }

        if let Some(type_name) = &self.type_name {
            p.nbsp();
            type_name.print(p)?;
        }

        if self.raw_default.is_some() {
            p.nbsp();
            p.word("using ");
            self.raw_default.as_deref()?.print(p)?;
        }

        if !self.fdwoptions.is_empty() {
            p.nbsp();
            print_create_generic_options(p, &self.fdwoptions);
        }

        for constraint in self.constraints.iter() {
            match constraint.node.as_ref().unwrap() {
                NodeEnum::Constraint(constraint) => {
                    p.nbsp();
                    constraint.print(p)?;
                }
                _ => unreachable!(),
            }
        }

        if self.coll_clause.is_some() {
            self.coll_clause.as_ref()?.print(p)?;
        }

        Some(())
    }
}

pub fn print_create_generic_options(_str: &mut fmt::Printer, _list: &[Node]) {
    todo!()
}
