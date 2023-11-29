use crate::create_stmt::node_col_label;
use crate::create_stmt::node_opt_indirection;
use crate::fmt;
use pg_query::protobuf::ColumnRef;
use pg_query::NodeEnum;

impl fmt::Print for ColumnRef {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        if let NodeEnum::AStar(node) = self.fields.first().unwrap().node.as_ref().unwrap() {
            node.print(p)?;
        } else if let NodeEnum::String(node) = self.fields.first().unwrap().node.as_ref().unwrap() {
            node_col_label(p, &node.sval);
        }

        node_opt_indirection(p, &self.fields, 1);

        Some(())
    }
}
