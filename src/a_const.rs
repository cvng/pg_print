use crate::fmt;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::AConst;

impl fmt::Print for AConst {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        Option::<Val>::print_in_context(&self.val, p, &fmt::Context::Constant)
    }
}
