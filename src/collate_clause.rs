use crate::fmt;
use pg_query::protobuf::CollateClause;

impl fmt::Print for CollateClause {
    fn print(&self, _p: &mut fmt::Printer) -> fmt::Result {
        todo!("{:?}", self)
    }
}
