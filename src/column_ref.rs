use crate::fmt;
use pg_query::protobuf::ColumnRef;
use pg_query::Node;
use pg_query::NodeEnum;

impl fmt::Print for ColumnRef {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        if let NodeEnum::AStar(node) = self.fields.first().unwrap().node.as_ref().unwrap() {
            node.print(p)?;
        } else if let NodeEnum::String(node) = self.fields.first().unwrap().node.as_ref().unwrap() {
            print_col_label(p, &node.sval);
        }

        print_opt_indirection(p, &self.fields, 1);

        Some(())
    }
}

fn print_col_label(str: &mut fmt::Printer, node: &str) {
    str.ident(node.to_owned());
}

fn print_opt_indirection(_str: &mut fmt::Printer, _list: &[Node], _offset: usize) {
    // for (i, item) in list.iter().enumerate().skip(offset) {}
}
