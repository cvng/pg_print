use crate::fmt;
use crate::rel_persistence::RelPersistence;
use pg_query::protobuf::CreateTableAsStmt;
use pg_query::NodeEnum;

impl fmt::Print for CreateTableAsStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
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
        .ok()?
        .print(p)?;

        self.objtype().print(p)?;

        if self.if_not_exists {
            p.word("if not exists ");
        }

        self.into.as_ref()?.print(p)?;
        p.word(" ");

        p.keyword("as ");

        match self.query.as_ref().unwrap().node.as_ref().unwrap() {
            NodeEnum::ExecuteStmt(node) => node.print(p)?,
            NodeEnum::SelectStmt(node) => node.print(p)?,
            _ => {}
        }

        p.word(" ");

        if self.into.is_some() && self.into.as_ref().unwrap().skip_data {
            p.word("with no data ");
        }

        Some(())
    }
}
