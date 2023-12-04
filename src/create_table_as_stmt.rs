use crate::fmt;
use pg_query::protobuf::CreateTableAsStmt;

impl fmt::Print for CreateTableAsStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.word("create ");

        p.opt_temp(
            self.into
                .as_ref()
                .unwrap()
                .rel
                .as_ref()
                .unwrap()
                .relpersistence
                .clone(),
        )?;

        self.objtype().print(p)?;

        if self.if_not_exists {
            p.word("if not exists ");
        }

        self.into.as_ref().unwrap().print(p)?;
        p.word(" ");

        p.word("as ");

        p.node(self.query.as_ref().unwrap());

        p.word(" ");

        if let Some(into) = self.into.as_deref() {
            if into.skip_data {
                p.word("with no data ");
            }
        }

        Ok(())
    }
}
