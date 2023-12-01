use crate::fmt;
use crate::fmt::gram;
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
                        [list.clone()].print(p)?;
                        p.word(")");
                        p.comma(i >= self.values_lists.len() - 1);
                    }

                    p.word(" ");
                }

                p.keyword("select ");

                if !self.target_list.is_empty() {
                    if !self.distinct_clause.is_empty() {
                        p.word("distinct ");

                        p.word("on (");
                        self.distinct_clause.print(p)?;
                        p.word(") ");
                    }

                    self.target_list.print(p)?;
                    p.word(" ");
                }

                gram::print_from_clause(p, &self.from_clause)?;
                gram::print_where_clause(p, self.where_clause.as_deref())?;
            }
            _ => todo!("{:?}", self.op()),
        };

        Ok(())
    }
}
