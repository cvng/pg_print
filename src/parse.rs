use crate::fmt::Printer;
use crate::stmt::RawStmt;

impl Printer {
    #[cfg(feature = "unstable")]
    pub fn parse(&mut self, parse: &parser::Parse) {
        self.cbox(0);
        for stmt in &parse.stmts {
            self.stmt(&RawStmt { stmt: &stmt.stmt });
        }
        self.end();
    }

    #[cfg(not(feature = "unstable"))]
    pub fn parse(&mut self, parse: &pg_query::protobuf::ParseResult) {
        self.cbox(0);
        for stmt in &parse.stmts {
            if let Some(stmt) = &stmt.stmt {
                if let Some(node) = &stmt.node {
                    self.stmt(&RawStmt { stmt: node });
                }
            }
        }
        self.end();
    }
}
