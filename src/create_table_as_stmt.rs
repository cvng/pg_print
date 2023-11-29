use crate::create_stmt::node_expr;
use crate::create_stmt::node_opt_temp;
use crate::create_stmt::node_range_var;
use crate::fmt;
use crate::fmt::DeparseNodeContext;
use crate::fmt::Print;
use crate::fmt::Printer;
use pg_query::protobuf::CreateTableAsStmt;
use pg_query::Node;
use pg_query::NodeEnum;

impl fmt::Print for CreateTableAsStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        p.keyword("create ");

        node_opt_temp(
            p,
            &self
                .into
                .as_ref()
                .unwrap()
                .rel
                .as_ref()
                .unwrap()
                .relpersistence,
        );

        self.objtype().print(p)?;

        if self.if_not_exists {
            p.word("if not exists ");
        }

        self.into.as_ref()?.print(p)?;
        p.word(" ");

        p.keyword("as ");

        match self.query.as_ref().unwrap().node.as_ref().unwrap() {
            NodeEnum::ExecuteStmt(node) => node.print(p)?,
            NodeEnum::SelectStmt(node) => node.print(p)?,
            _ => {}
        }

        p.word(" ");

        if self.into.is_some() && self.into.as_ref().unwrap().skip_data {
            p.word("with no data ");
        }

        Some(())
    }
}

pub fn node_from_clause(str: &mut Printer, list: &[Node]) {
    if !list.is_empty() {
        str.keyword("from ");
        node_from_list(str, list);
        str.word(" ");
    }
}

fn node_from_list(str: &mut Printer, list: &[Node]) {
    for (i, item) in list.iter().enumerate() {
        node_table_ref(str, item);
        str.comma(i >= list.len() - 1);
    }
}

fn node_table_ref(str: &mut Printer, node: &Node) {
    match node.node.as_ref().unwrap() {
        NodeEnum::RangeVar(node) => node_range_var(str, node, DeparseNodeContext::None),
        _ => todo!("{:?}", node),
    }
}

pub fn node_where_clause(str: &mut Printer, node: Option<&Node>) {
    if let Some(node) = node {
        str.keyword("where ");
        node_expr(str, Some(node));
        str.word(" ");
    }
}

pub fn node_target_list(str: &mut Printer, list: &[Node]) {
    for (i, entry) in list.iter().enumerate() {
        if let NodeEnum::ResTarget(node) = entry.node.as_ref().unwrap() {
            if node.val.is_none() {
            } else if let NodeEnum::ColumnRef(node) =
                node.val.as_ref().unwrap().node.as_ref().unwrap()
            {
                node.print(str);
            } else {
                node_expr(str, node.val.as_deref());
            }

            if !node.name.is_empty() {
                str.word(" as ");
                str.ident(node.name.clone());
            }

            str.comma(i >= list.len() - 1);
        }
    }
}
