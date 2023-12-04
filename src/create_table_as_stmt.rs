use crate::fmt::Printer;
use pg_query::protobuf::CreateTableAsStmt;
use pg_query::NodeEnum;

impl Printer {
    pub fn create_table_as_stmt(&mut self, n: &CreateTableAsStmt) {
        self.word("create ");

        self.opt_temp(
            n.into
                .as_ref()
                .unwrap()
                .rel
                .as_ref()
                .unwrap()
                .relpersistence
                .clone(),
        );

        self.object_type(&n.objtype());

        if n.if_not_exists {
            self.word("if not exists ");
        }

        self.into_clause(n.into.as_ref().unwrap());
        self.word(" ");

        self.word("as ");

        if let NodeEnum::SelectStmt(query) = &n.query.as_ref().unwrap().node.as_ref().unwrap() {
            self.select_stmt(query);
        }

        self.word(" ");

        if let Some(into) = n.into.as_deref() {
            if into.skip_data {
                self.word("with no data ");
            }
        }
    }
}
