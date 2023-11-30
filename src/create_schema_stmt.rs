use crate::fmt;
use crate::utils::print_expr_list;
use pg_query::protobuf::CreateSchemaStmt;

impl fmt::Print for CreateSchemaStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.keyword("create schema ");

        if self.if_not_exists {
            p.keyword("if not exists ");
        }

        p.ident(self.schemaname.clone());

        if let Some(authrole) = &self.authrole {
            p.keyword("authorization ");
            authrole.print(p)?;
            p.nbsp();
        }

        print_expr_list(p, &self.schema_elts)?;

        Ok(())
    }
}
