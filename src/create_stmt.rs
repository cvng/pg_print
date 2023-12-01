use crate::fmt;
use crate::INDENT;
use pg_query::protobuf::CreateStmt;
use pg_query::NodeEnum;

impl fmt::Print for CreateStmt {
    fn print_in_context(&self, p: &mut fmt::Printer, ctx: &fmt::Context) -> fmt::Result {
        p.cbox(INDENT);
        p.keyword("create ");

        if matches!(ctx, fmt::Context::ForeignRelation) {
            p.keyword("foreign ");
        }

        p.opt_temp(self.relation.as_ref().unwrap().relpersistence.clone())?;

        p.keyword("table ");

        if self.if_not_exists {
            p.keyword("if not exists ");
        }

        if let Some(relation) = &self.relation {
            relation.print(p)?;
            p.nbsp();
        }

        if let Some(of_typename) = &self.of_typename {
            p.keyword("of ");
            of_typename.print(p)?;
            p.nbsp();
        }

        if self.partbound.is_some() {
            p.keyword("partition of ");
            self.inh_relations
                .first()
                .and_then(|node| match node.node.as_ref().unwrap() {
                    NodeEnum::RangeVar(node) => Some(node),
                    _ => None,
                })
                .unwrap()
                .print(p)?;
            p.word(" ");
        }

        if !self.table_elts.is_empty() {
            p.word("(");
            p.hardbreak_if_nonempty();
            for (i, elt) in self.table_elts.iter().enumerate() {
                elt.print(p)?;
                if i < self.table_elts.len() - 1 {
                    p.word(",");
                }
                p.hardbreak();
            }
            p.offset(-INDENT);
            p.end();
            p.word(")");
        } else if self.partbound.is_none() && self.of_typename.is_none() {
            p.word("()");
        };

        if let Some(partbound) = &self.partbound {
            partbound.print(p)?;
            p.word(" ");
        } else {
            p.opt_inherit(&self.inh_relations)?;
        }

        p.opt_with(&self.options)?;

        self.oncommit().print(p)?;

        if !self.tablespacename.is_empty() {
            p.keyword("tablespace ");
            p.ident(self.tablespacename.clone());
        }

        p.hardbreak();

        Ok(())
    }
}
