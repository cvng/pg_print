use crate::fmt;
use pg_query::protobuf::ParamRef;

impl fmt::Print for ParamRef {
    fn print(&self, _p: &mut fmt::Printer) {
        todo!("{:?}", self)
    }
}
