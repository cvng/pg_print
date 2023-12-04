use crate::fmt::Printer;
use crate::INDENT;
use pg_query::protobuf::CreateForeignTableStmt;
use pg_query::Node;
use pg_query::NodeEnum;

impl Printer {
    pub fn create_foreign_table_stmt(&mut self, n: &CreateForeignTableStmt) {
        self.word("create foreign table ");
        self.qualified_name(&Node {
            node: Some(NodeEnum::RangeVar(
                n.base_stmt
                    .as_ref()
                    .unwrap()
                    .relation
                    .as_ref()
                    .unwrap()
                    .clone(), // TODO: expensive clone (size = 152)
            )),
        });
        self.nbsp();

        if !n.base_stmt.as_ref().unwrap().table_elts.is_empty() {
            self.cbox(INDENT);
            self.word("(");
            self.hardbreak_if_nonempty();
            self.print_list(&n.base_stmt.as_ref().unwrap().table_elts);
            self.hardbreak();
            self.offset(-INDENT);
            self.end();
            self.word(")");
        }

        self.opt_inherit(&n.base_stmt.as_ref().unwrap().inh_relations);

        self.hardbreak();
        self.word("server ");
        self.name(n.servername.clone());
        self.nbsp();

        self.create_generic_options(&n.options);
    }
}
