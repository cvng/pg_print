use crate::fmt;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::Integer;

impl fmt::Print for Integer {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        Some(Val::Ival(self.clone())).print(p)
    }
}
