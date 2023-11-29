use crate::fmt;
use pg_query::protobuf::ExecuteStmt;
use crate::utils::expr_list;

impl fmt::Print for ExecuteStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        p.word("execute ");
        p.ident(self.name.clone());

        if !self.params.is_empty() {
            p.word("(");
            expr_list(p, &self.params);
            p.word(")");
        }

        Some(())
    }
}
