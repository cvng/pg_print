use crate::fmt;
use pg_query::protobuf::CreateSchemaStmt;

impl fmt::Print for CreateSchemaStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.keyword("create schema ");
        p.optional_keyword("if not exists ", self.if_not_exists);
        p.ident(self.schemaname.clone());
        self.authrole.as_ref().and_then(|a| {
            p.keyword("authorization ");
            a.print(p).ok();
            p.nbsp()
        });
        self.schema_elts.print(p)
    }
}
