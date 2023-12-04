use crate::fmt;
use crate::fmt::Context;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::Integer;

impl fmt::Print for Integer {
    fn print(&self, p: &mut fmt::Printer) {
        self.opt_val(Some(&Val::Ival(self.clone())), &Context::None);
    }
}
