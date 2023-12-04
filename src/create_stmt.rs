use crate::fmt::Printer;
use crate::INDENT;
use pg_query::protobuf::CreateStmt;
use pg_query::NodeEnum;

impl Printer {
    pub fn create_stmt(&mut self, n: &CreateStmt) {
        self.cbox(INDENT);
        self.word("create ");

        self.opt_temp(n.relation.as_ref().unwrap().relpersistence.clone());

        self.word("table ");

        if n.if_not_exists {
            self.word("if not exists ");
        }

        if let Some(relation) = &n.relation {
            self.range_var(relation);
            self.nbsp();
        }

        if let Some(of_typename) = &n.of_typename {
            self.word("of ");
            self.type_name(of_typename);
            self.nbsp();
        }

        if n.partbound.is_some() {
            self.word("partition of ");
            self.range_var(
                n.inh_relations
                    .first()
                    .and_then(|node| match node.node.as_ref().unwrap() {
                        NodeEnum::RangeVar(node) => Some(node),
                        _ => None,
                    })
                    .unwrap(),
            );
            self.word(" ");
        }

        if !n.table_elts.is_empty() {
            self.word("(");
            self.hardbreak_if_nonempty();
            for (i, elt) in n.table_elts.iter().enumerate() {
                self.node(elt);
                if i < n.table_elts.len() - 1 {
                    self.word(",");
                }
                self.hardbreak();
            }
            self.offset(-INDENT);
            self.end();
            self.word(")");
        } else if n.partbound.is_none() && n.of_typename.is_none() {
            self.word("()");
        };

        if let Some(partbound) = &n.partbound {
            self.partition_bound_spec(partbound);
            self.word(" ");
        } else {
            self.opt_inherit(&n.inh_relations);
        }

        self.opt_with(&n.options);

        self.on_commit_action(&n.oncommit());

        if !n.tablespacename.is_empty() {
            self.word("tablespace ");
            self.ident(n.tablespacename.clone());
        }

        self.hardbreak();
    }
}
