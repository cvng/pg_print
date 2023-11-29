use crate::create_stmt::node_value;
use crate::fmt;
use crate::fmt::DeparseNodeContext;
use pg_query::protobuf::AConst;

impl fmt::Print for AConst {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        node_value(p, self.val.as_ref(), DeparseNodeContext::Constant);
        Some(())
    }
}
