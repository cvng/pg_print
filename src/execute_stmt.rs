use crate::fmt::Printer;
use pg_query::protobuf::ExecuteStmt;

impl Printer {
    pub fn execute_stmt(&mut self, n: &ExecuteStmt) {
        self.word("execute ");
        self.ident(n.name.clone());

        if !n.params.is_empty() {
            self.word("(");
            self.print_list(&n.params);
            self.word(")");
        }
    }
}
