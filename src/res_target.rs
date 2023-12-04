use crate::fmt::Printer;
use pg_query::protobuf::ResTarget;
use pg_query::NodeEnum;

impl Printer {
    pub fn res_target(&mut self, n: &ResTarget) {
        if n.val.is_none() {
        } else if let NodeEnum::ColumnRef(node) = n.val.as_ref().unwrap().node.as_ref().unwrap() {
            self.column_ref(node);
        } else {
            self.node(n.val.as_deref().unwrap());
        }

        if !n.name.is_empty() {
            self.word(" as ");
            self.ident(n.name.clone());
        }
    }
}
