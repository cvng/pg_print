use crate::fmt;
use crate::fmt::Print;
use crate::utils::print_expr_list;
use pg_query::protobuf::SelectStmt;
use pg_query::protobuf::SetOperation;
use pg_query::Node;

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
                        print_expr_list(p, &[list.clone()])?;
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
                        print_expr_list(p, &self.distinct_clause)?;
                        p.word(") ");
                    }

                    print_expr_list(p, &self.target_list)?;
                    p.word(" ");
                }

                print_from_clause(p, &self.from_clause)?;
                print_where_clause(p, self.where_clause.as_deref())?;
            }
            _ => todo!("{:?}", self.op()),
        };

        Ok(())
    }
}

fn print_from_clause(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    if !list.is_empty() {
        p.keyword("from ");

        for (i, item) in list.iter().enumerate() {
            item.print(p)?;
            p.comma(i >= list.len() - 1);
        }
        p.word(" ");
    }

    Ok(())
}

fn print_where_clause(p: &mut fmt::Printer, node: Option<&Node>) -> fmt::Result {
    if let Some(node) = node {
        p.keyword("where ");
        node.print(p)?;
        p.word(" ");
    }

    Ok(())
}
