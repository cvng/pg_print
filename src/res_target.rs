use crate::fmt;
use pg_query::protobuf::ResTarget;
use pg_query::NodeEnum;

impl fmt::Print for ResTarget {
    fn print(&self, p: &mut fmt::Printer) {
        if self.val.is_none() {
        } else if let NodeEnum::ColumnRef(node) = self.val.as_ref().unwrap().node.as_ref().unwrap()
        {
            node.print(p);
        } else {
            p.node(self.val.as_deref().unwrap());
        }

        if !self.name.is_empty() {
            p.word(" as ");
            p.ident(self.name.clone());
        }
    }
}
