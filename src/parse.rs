use crate::fmt::Printer;
use crate::stmt::RawStmt;
use parser::Parse;

impl Printer {
    pub fn parse(&mut self, parse: &Parse) {
        self.cbox(0);
        for stmt in &parse.stmts {
            self.stmt(&RawStmt { stmt: &stmt.stmt });
        }
        self.end();
    }
}
