use crate::create_stmt::node_expr_list;
use crate::fmt;
use pg_query::protobuf::ExecuteStmt;

impl fmt::Print for ExecuteStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        p.word("execute ");
        p.ident(self.name.clone());

        if !self.params.is_empty() {
            p.word("(");
            node_expr_list(p, &self.params);
            p.word(")");
        }

        Some(())
    }
}
