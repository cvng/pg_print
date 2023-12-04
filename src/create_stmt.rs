use crate::fmt;
use crate::INDENT;
use pg_query::protobuf::CreateStmt;
use pg_query::NodeEnum;

impl fmt::Print for CreateStmt {
    fn print(&self, p: &mut fmt::Printer) {
        self.cbox(INDENT);
        self.word("create ");

        self.opt_temp(self.relation.as_ref().unwrap().relpersistence.clone());

        self.word("table ");

        if self.if_not_exists {
            self.word("if not exists ");
        }

        if let Some(relation) = &self.relation {
            relation.print(p);
            self.nbsp();
        }

        if let Some(of_typename) = &self.of_typename {
            self.word("of ");
            of_typename.print(p);
            self.nbsp();
        }

        if self.partbound.is_some() {
            self.word("partition of ");
            self.inh_relations
                .first()
                .and_then(|node| match node.node.as_ref().unwrap() {
                    NodeEnum::RangeVar(node) => Some(node),
                    _ => None,
                })
                .unwrap()
                .print(p);
            self.word(" ");
        }

        if !self.table_elts.is_empty() {
            self.word("(");
            self.hardbreak_if_nonempty();
            for (i, elt) in self.table_elts.iter().enumerate() {
                self.node(elt);
                if i < self.table_elts.len() - 1 {
                    self.word(",");
                }
                self.hardbreak();
            }
            self.offset(-INDENT);
            self.end();
            self.word(")");
        } else if self.partbound.is_none() && self.of_typename.is_none() {
            self.word("()");
        };

        if let Some(partbound) = &self.partbound {
            partbound.print(p);
            self.word(" ");
        } else {
            self.opt_inherit(&self.inh_relations);
        }

        self.opt_with(&self.options);

        self.oncommit().print(p);

        if !self.tablespacename.is_empty() {
            self.word("tablespace ");
            self.ident(self.tablespacename.clone());
        }

        self.hardbreak();
    }
}
