use crate::fmt;
use pg_query::protobuf::CreateSchemaStmt;

impl fmt::Print for CreateSchemaStmt {
    fn print(&self, p: &mut fmt::Printer) {
        p.word("create schema ");
        p.optional_word("if not exists ", self.if_not_exists);
        p.ident(self.schemaname.clone());

        if let Some(authrole) = &self.authrole {
            p.word("authorization ");
            authrole.print(p);
            p.nbsp();
        }

        p.print_list(&self.schema_elts);
    }
}
