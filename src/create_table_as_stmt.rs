use crate::algorithm::Printer;
use crate::create_stmt::node_column_list;
use crate::create_stmt::node_column_ref;
use crate::create_stmt::node_expr;
use crate::create_stmt::node_expr_list;
use crate::create_stmt::node_on_commit_action;
use crate::create_stmt::node_opt_temp;
use crate::create_stmt::node_opt_with;
use crate::create_stmt::node_range_var;
use crate::create_stmt::DeparseNodeContext;
use pg_query::protobuf::ColumnRef;
use pg_query::protobuf::CreateTableAsStmt;
use pg_query::protobuf::ExecuteStmt;
use pg_query::protobuf::IntoClause;
use pg_query::protobuf::ObjectType;
use pg_query::protobuf::OnCommitAction;
use pg_query::protobuf::ResTarget;
use pg_query::protobuf::SelectStmt;
use pg_query::protobuf::SetOperation;
use pg_query::protobuf::TargetEntry;
use pg_query::protobuf::WithClause;
use pg_query::Node;
use pg_query::NodeEnum;

pub fn node_create_table_as_stmt(str: &mut Printer, node: &CreateTableAsStmt) {
    str.keyword("create ");

    node_opt_temp(
        str,
        &node
            .into
            .as_ref()
            .unwrap()
            .rel
            .as_ref()
            .unwrap()
            .relpersistence,
    );

    match node.objtype() {
        ObjectType::ObjectTable => str.keyword("table "),
        ObjectType::ObjectMatview => str.keyword("materialized view "),
        _ => unimplemented!("{:?}", node.objtype()),
    }

    if node.if_not_exists {
        str.word("if not exists ");
    }

    node_into_clause(str, node.into.as_ref().unwrap());
    str.word(" ");

    str.keyword("as ");

    match node.query.as_ref().unwrap().node.as_ref().unwrap() {
        NodeEnum::ExecuteStmt(node) => node_execute_stmt(str, node),
        NodeEnum::SelectStmt(node) => node_select_stmt(str, node),
        _ => {}
    }

    str.word(" ");

    if node.into.is_some() && node.into.as_ref().unwrap().skip_data {
        str.word("with no data ");
    }
}

fn node_into_clause(str: &mut Printer, node: &IntoClause) {
    node_range_var(str, node.rel.as_ref().unwrap(), DeparseNodeContext::None);

    if !node.col_names.is_empty() {
        str.word(" (");
        node_column_list(str, &node.col_names);
        str.word(")");
    }

    if !node.access_method.is_empty() {
        str.word("using ");
        str.ident(node.access_method.clone());
        str.word(" ");
    }

    node_opt_with(str, &node.options);

    node_on_commit_action(str, &node.on_commit());

    if !node.table_space_name.is_empty() {
        str.word("tablespace ");
        str.ident(node.table_space_name.clone());
        str.word(" ");
    }
}

fn node_execute_stmt(str: &mut Printer, node: &ExecuteStmt) {
    str.word("execute ");
    str.ident(node.name.clone());

    if !node.params.is_empty() {
        str.word("(");
        node_expr_list(str, &node.params);
        str.word(")");
    }
}

fn node_select_stmt(str: &mut Printer, node: &SelectStmt) {
    if let Some(with_clause) = &node.with_clause {
        node_with_clause(str, with_clause);
        str.word(" ");
    }

    match &node.op() {
        SetOperation::SetopNone => {
            if !node.values_lists.is_empty() {
                str.word("values ");

                for (i, list) in node.values_lists.iter().enumerate() {
                    str.word("(");
                    node_expr_list(str, &[list.clone()]);
                    str.word(")");
                    str.comma(i >= node.values_lists.len() - 1);
                }

                str.word(" ");
            }

            str.keyword("select ");

            if !node.target_list.is_empty() {
                if !node.distinct_clause.is_empty() {
                    str.word("distinct ");

                    str.word("on (");
                    node_expr_list(str, &node.distinct_clause);
                    str.word(") ");
                }

                node_target_list(str, &node.target_list);
                str.word(" ");
            }

            node_from_clause(str, &node.from_clause);
            node_where_clause(str, node.where_clause.as_deref());
        }
        _ => todo!("{:?}", node.op()),
    }
}

fn node_from_clause(str: &mut Printer, list: &[Node]) {
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

fn node_where_clause(str: &mut Printer, node: Option<&Node>) {
    if let Some(node) = node {
        str.keyword("where ");
        node_expr(str, Some(node));
        str.word(" ");
    }
}

fn node_with_clause(str: &mut Printer, node: &WithClause) {
    str.word("with ");

    if node.recursive {
        str.word("recursive ");
    }

    todo!("{:?}", &node);
}

fn node_target_list(str: &mut Printer, list: &[Node]) {
    for (i, entry) in list.iter().enumerate() {
        if let NodeEnum::ResTarget(node) = entry.node.as_ref().unwrap() {
            if node.val.is_none() {
            } else if let NodeEnum::ColumnRef(node) =
                node.val.as_ref().unwrap().node.as_ref().unwrap()
            {
                node_column_ref(str, node);
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
