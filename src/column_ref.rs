use crate::fmt;
use pg_query::protobuf::ColumnRef;
use pg_query::Node;
use pg_query::NodeEnum;

impl fmt::Print for ColumnRef {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        if let NodeEnum::AStar(node) = self.fields.first().unwrap().node.as_ref().unwrap() {
            p.a_star(node);
        } else if let NodeEnum::String(node) = self.fields.first().unwrap().node.as_ref().unwrap() {
            print_col_label(p, &node.sval);
        }

        print_opt_indirection(p, &self.fields, 1);

        Ok(())
    }
}

fn print_col_label(p: &mut fmt::Printer, node: &str) {
    p.ident(node.to_owned());
}

fn print_opt_indirection(_p: &mut fmt::Printer, _list: &[Node], _offset: usize) {
    // for (i, item) in list.iter().enumerate().skip(offset) {}
}
