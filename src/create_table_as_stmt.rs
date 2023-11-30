use crate::fmt;
use crate::rel_persistence::RelPersistence;
use pg_query::protobuf::CreateTableAsStmt;
use pg_query::protobuf::IntoClause;

impl fmt::Print for CreateTableAsStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.keyword("create ");

        RelPersistence::try_from(
            self.into
                .as_ref()
                .unwrap()
                .rel
                .as_ref()
                .unwrap()
                .relpersistence
                .clone(),
        )
        .unwrap()
        .print(p)?;

        self.objtype().print(p)?;

        if self.if_not_exists {
            p.word("if not exists ");
        }

        self.into.as_ref().unwrap().print(p)?;
        p.word(" ");

        p.keyword("as ");

        self.query.as_ref().unwrap().print(p)?;

        p.word(" ");

        if let Some(IntoClause {
            skip_data: true, ..
        }) = self.into.as_deref()
        {
            p.word("with no data ");
        }

        Ok(())
    }
}
