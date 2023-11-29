use crate::fmt;
use pg_query::protobuf::AConst;

impl fmt::Print for AConst {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        self.val.print_in_context(p, &fmt::Context::Constant)
    }
}
