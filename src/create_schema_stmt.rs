use crate::fmt::Print;
use crate::fmt::Printer;
use pg_query::protobuf::CreateSchemaStmt;

impl Printer {
    pub fn create_schema_stmt(&mut self, n: &CreateSchemaStmt) {
        self.word("create schema ");
        self.optional_word("if not exists ", n.if_not_exists);
        self.ident(n.schemaname.clone());

        if let Some(authrole) = &n.authrole {
            self.word("authorization ");
            authrole.print(self);
            self.nbsp();
        }

        self.print_list(&n.schema_elts);
    }
}
