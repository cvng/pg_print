use crate::fmt::Printer;
use pg_query::protobuf::ColumnRef;
use pg_query::Node;
use pg_query::NodeEnum;

impl Printer {
    pub fn column_ref(&mut self, n: &ColumnRef) {
        if let NodeEnum::AStar(node) = n.fields.first().unwrap().node.as_ref().unwrap() {
            self.a_star(node);
        } else if let NodeEnum::String(node) = n.fields.first().unwrap().node.as_ref().unwrap() {
            self.col_label(&node.sval);
        }

        self.opt_indirection(&n.fields, 1);
    }

    pub fn col_label(&mut self, node: &str) {
        self.ident(node.to_owned());
    }

    pub fn opt_indirection(&mut self, _list: &[Node], _offset: usize) {
        // for (i, item) in list.iter().enumerate().skip(offset) {}
    }
}
