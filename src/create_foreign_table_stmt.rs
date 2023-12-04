use crate::fmt;
use crate::INDENT;
use pg_query::protobuf::CreateForeignTableStmt;
use pg_query::Node;
use pg_query::NodeEnum;

impl fmt::Print for CreateForeignTableStmt {
    fn print(&self, p: &mut fmt::Printer) {
        self.word("create foreign table ");
        self.qualified_name(&Node {
            node: Some(NodeEnum::RangeVar(
                self.base_stmt
                    .as_ref()
                    .unwrap()
                    .relation
                    .as_ref()
                    .unwrap()
                    .clone(), // TODO: expensive clone (size = 152)
            )),
        });
        self.nbsp();

        if !self.base_stmt.as_ref().unwrap().table_elts.is_empty() {
            self.cbox(INDENT);
            self.word("(");
            self.hardbreak_if_nonempty();
            self.print_list(&self.base_stmt.as_ref().unwrap().table_elts);
            self.hardbreak();
            self.offset(-INDENT);
            self.end();
            self.word(")");
        }

        self.opt_inherit(&self.base_stmt.as_ref().unwrap().inh_relations);

        self.hardbreak();
        self.word("server ");
        self.name(self.servername.clone());
        self.nbsp();

        self.create_generic_options(&self.options);
    }
}
