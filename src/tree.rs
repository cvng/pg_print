use crate::algorithm::Printer;
use pg_query::protobuf::ParseResult;

impl Printer {
    pub fn tree(&mut self, tree: &ParseResult) {
        self.cbox(0);
        for stmt in &tree.stmts {
            self.stmt(stmt);
        }
        self.end();
    }
}
