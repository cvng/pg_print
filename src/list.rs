use crate::fmt::Printer;
use pg_query::protobuf::List;

impl Printer {
    pub fn list(&self, n: &List) {
        for item in &n.items {
            self.node(item);
        }
    }
}
