use crate::fmt::Printer;
use pg_query::protobuf::ParseResult;
use pg_query::protobuf::RawStmt;

impl Printer {
    pub fn tree(&mut self, tree: &ParseResult) {
        self.cbox(0);
        for stmt in &tree.stmts {
            self.stmt(stmt);
        }
        self.end();
    }

    fn stmt(&mut self, stmt: &RawStmt) {
        if let Some(stmt) = &stmt.stmt {
            self.node(stmt);
        }
    }
}
