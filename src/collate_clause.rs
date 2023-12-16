use crate::fmt::Printer;
use pg_query::protobuf::CollateClause;
use pg_query::NodeEnum;

impl Printer {
    pub fn opt_collate_clause(&mut self, n: Option<&CollateClause>) {
        if let Some(n) = n {
            self.collate_clause(n)
        }
    }

    pub fn collate_clause(&mut self, n: &CollateClause) {
        if let Some(arg) = &n.arg {
            let need_parens = matches!(arg.node.as_ref().unwrap(), NodeEnum::AExpr(_));

            self.optional_word("(", need_parens);
            self.node(arg);
            self.optional_word(")", need_parens);
            self.nbsp();
        }

        self.word("collate ");
        self.any_name(&n.collname);
    }
}
