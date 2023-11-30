use crate::fmt;
use pg_query::protobuf::ExecuteStmt;

impl fmt::Print for ExecuteStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.word("execute ");
        p.ident(self.name.clone());

        if !self.params.is_empty() {
            p.word("(");
            self.params.print(p)?;
            p.word(")");
        }

        Ok(())
    }
}
