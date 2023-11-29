use crate::fmt;
use crate::fmt::Print;
use crate::utils::print_expr_list;
use pg_query::protobuf::SelectStmt;
use pg_query::protobuf::SetOperation;
use pg_query::Node;
use pg_query::NodeEnum;

impl fmt::Print for SelectStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
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
                        print_expr_list(p, &[list.clone()]);
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
                        print_expr_list(p, &self.distinct_clause);
                        p.word(") ");
                    }

                    target_list(p, &self.target_list);
                    p.word(" ");
                }

                from_clause(p, &self.from_clause);
                where_clause(p, self.where_clause.as_deref());
            }
            _ => todo!("{:?}", self.op()),
        };

        Some(())
    }
}

fn from_clause(p: &mut fmt::Printer, list: &[Node]) {
    if !list.is_empty() {
        p.keyword("from ");

        for (i, item) in list.iter().enumerate() {
            item.print(p);
            p.comma(i >= list.len() - 1);
        }
        p.word(" ");
    }
}

fn where_clause(p: &mut fmt::Printer, node: Option<&Node>) {
    if let Some(node) = node {
        p.keyword("where ");
        node.print(p);
        p.word(" ");
    }
}

fn target_list(p: &mut fmt::Printer, list: &[Node]) {
    for (i, entry) in list.iter().enumerate() {
        if let NodeEnum::ResTarget(node) = entry.node.as_ref().unwrap() {
            if node.val.is_none() {
            } else if let NodeEnum::ColumnRef(node) =
                node.val.as_ref().unwrap().node.as_ref().unwrap()
            {
                node.print(p);
            } else {
                node.val.as_deref().unwrap().print(p);
            }

            if !node.name.is_empty() {
                p.word(" as ");
                p.ident(node.name.clone());
            }

            p.comma(i >= list.len() - 1);
        }
    }
}
