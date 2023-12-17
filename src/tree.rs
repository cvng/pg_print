use crate::algo::Printer;
use crate::stmt::RawStmt;
#[cfg(feature = "unstable")]
use parser::Parse as ParseResult;
#[cfg(not(feature = "unstable"))]
use pg_query::protobuf::ParseResult;

impl Printer {
    #[cfg(feature = "unstable")]
    pub fn tree(&mut self, tree: &ParseResult) {
        self.cbox(0);
        for stmt in &tree.stmts {
            self.stmt(&RawStmt { stmt: &stmt.stmt });
        }
        self.end();
    }

    #[cfg(not(feature = "unstable"))]
    pub fn tree(&mut self, tree: &ParseResult) {
        self.cbox(0);
        for stmt in &tree.stmts {
            if let Some(stmt) = &stmt.stmt {
                if let Some(node) = &stmt.node {
                    self.stmt(&RawStmt { stmt: node });
                }
            }
        }
        self.end();
    }
}
