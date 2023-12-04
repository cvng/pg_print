use crate::fmt;
use pg_query::protobuf::SelectStmt;
use pg_query::protobuf::SetOperation;

impl fmt::Print for SelectStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        if let Some(with_clause) = &self.with_clause {
            with_clause.print(p)?;
            p.word(" ");
        }

        match &self.op() {
            SetOperation::SetopNone => {
                if !self.values_lists.is_empty() {
                    p.word("values ");

                    for (i, list) in self.values_lists.iter().enumerate() {
                        p.word("(");
                        p.print_list(&[list.clone()]);
                        p.word(")");
                        p.trailing_comma(i >= self.values_lists.len() - 1);
                    }

                    p.word(" ");
                }

                p.word("select ");

                if !self.target_list.is_empty() {
                    if !self.distinct_clause.is_empty() {
                        p.word("distinct ");

                        p.word("on (");
                        p.print_list(&self.distinct_clause);
                        p.word(") ");
                    }

                    p.print_list(&self.target_list);
                    p.word(" ");
                }

                p.from_clause(&self.from_clause)?;
                p.where_clause(self.where_clause.as_deref())?;
            }
            _ => todo!("{:?}", self.op()),
        };

        Ok(())
    }
}
