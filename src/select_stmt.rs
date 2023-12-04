use crate::fmt::Printer;
use pg_query::protobuf::SelectStmt;
use pg_query::protobuf::SetOperation;

impl Printer {
    pub fn select_stmt(&mut self, n: &SelectStmt) {
        if let Some(with_clause) = &n.with_clause {
            self.with_clause(with_clause);
            self.word(" ");
        }

        match &n.op() {
            SetOperation::SetopNone => {
                if !n.values_lists.is_empty() {
                    self.word("values ");

                    for (i, list) in n.values_lists.iter().enumerate() {
                        self.word("(");
                        self.print_list(&[list.clone()]);
                        self.word(")");
                        self.trailing_comma(i >= n.values_lists.len() - 1);
                    }

                    self.word(" ");
                }

                self.word("select ");

                if !n.target_list.is_empty() {
                    if !n.distinct_clause.is_empty() {
                        self.word("distinct ");

                        self.word("on (");
                        self.print_list(&n.distinct_clause);
                        self.word(") ");
                    }

                    self.print_list(&n.target_list);
                    self.word(" ");
                }

                self.from_clause(&n.from_clause);
                self.where_clause(n.where_clause.as_deref());
            }
            _ => todo!(),
        }
    }
}
