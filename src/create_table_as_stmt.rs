use crate::fmt;
use pg_query::protobuf::CreateTableAsStmt;

impl fmt::Print for CreateTableAsStmt {
    fn print(&self, p: &mut fmt::Printer) {
        self.word("create ");

        self.opt_temp(
            self.into
                .as_ref()
                .unwrap()
                .rel
                .as_ref()
                .unwrap()
                .relpersistence
                .clone(),
        );

        self.objtype().print(p);

        if self.if_not_exists {
            self.word("if not exists ");
        }

        self.into.as_ref().unwrap().print(p);
        self.word(" ");

        self.word("as ");

        self.node(self.query.as_ref().unwrap());

        self.word(" ");

        if let Some(into) = self.into.as_deref() {
            if into.skip_data {
                self.word("with no data ");
            }
        }
    }
}
