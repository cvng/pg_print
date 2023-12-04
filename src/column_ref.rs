use crate::fmt;
use crate::fmt::Printer;
use pg_query::protobuf::ColumnRef;
use pg_query::Node;
use pg_query::NodeEnum;

impl fmt::Print for ColumnRef {
    fn print(&self, p: &mut fmt::Printer) {
        if let NodeEnum::AStar(node) = self.fields.first().unwrap().node.as_ref().unwrap() {
            p.a_star(node);
        } else if let NodeEnum::String(node) = self.fields.first().unwrap().node.as_ref().unwrap() {
            p.col_label(&node.sval);
        }

        p.opt_indirection(&self.fields, 1);
    }
}

impl Printer {
    pub fn col_label(&mut self, node: &str) {
        self.ident(node.to_owned());
    }

    pub fn opt_indirection(&mut self, _list: &[Node], _offset: usize) {
        // for (i, item) in list.iter().enumerate().skip(offset) {}
    }
}
