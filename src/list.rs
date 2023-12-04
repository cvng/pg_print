use crate::fmt;
use pg_query::protobuf::List;

impl fmt::Print for List {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        for item in &self.items {
            p.node(item);
        }

        Ok(())
    }
}
