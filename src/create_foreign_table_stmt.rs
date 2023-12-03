use crate::fmt;
use crate::INDENT;
use pg_query::protobuf::CreateForeignTableStmt;
use pg_query::Node;
use pg_query::NodeEnum;

impl fmt::Print for CreateForeignTableStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.word("create foreign table ");
        p.qualified_name(&Node {
            node: Some(NodeEnum::RangeVar(
                self.base_stmt
                    .as_ref()
                    .unwrap()
                    .relation
                    .as_ref()
                    .unwrap()
                    .clone(), // TODO: expensive clone (size = 152)
            )),
        })?;
        p.nbsp();

        if !self.base_stmt.as_ref().unwrap().table_elts.is_empty() {
            p.cbox(INDENT);
            p.word("(");
            p.hardbreak_if_nonempty();
            self.base_stmt.as_ref().unwrap().table_elts.print(p)?;
            p.hardbreak();
            p.offset(-INDENT);
            p.end();
            p.word(")");
        }

        p.opt_inherit(&self.base_stmt.as_ref().unwrap().inh_relations)?;

        p.hardbreak();
        p.word("server ");
        p.name(self.servername.clone());
        p.nbsp();

        p.create_generic_options(&self.options)?;
        Ok(())
    }
}
