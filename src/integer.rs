use crate::fmt::Context;
use crate::fmt::Printer;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::Integer;

impl Printer {
    pub fn integer(&mut self, n: &Integer) {
        self.opt_val(Some(&Val::Ival(n.clone())), &Context::None);
    }
}
