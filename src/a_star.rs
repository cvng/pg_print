use crate::fmt;
use pg_query::protobuf::AStar;

impl fmt::Print for AStar {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.word("*");
        Ok(())
    }
}
