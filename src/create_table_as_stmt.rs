use crate::algorithm::Printer;
use crate::create_stmt::node_column_list;
use crate::create_stmt::node_expr_list;
use crate::create_stmt::node_on_commit_action;
use crate::create_stmt::node_opt_temp;
use crate::create_stmt::node_opt_with;
use crate::create_stmt::node_range_var;
use crate::create_stmt::DeparseNodeContext;
use pg_query::protobuf::CreateTableAsStmt;
use pg_query::protobuf::ExecuteStmt;
use pg_query::protobuf::IntoClause;
use pg_query::protobuf::ObjectType;
use pg_query::protobuf::OnCommitAction;
use pg_query::protobuf::SelectStmt;
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

    str.word("as ");

    match node.query.as_ref().unwrap().node.as_ref().unwrap() {
        NodeEnum::ExecuteStmt(node) => node_execute_stmt(str, node),
        NodeEnum::SelectStmt(node) => node_select_stmt(str, node),
        _ => {}
    }

    str.word(" ");

    if (node.into.is_some()) {
        str.word("with no data ");
    }
}

fn node_into_clause(str: &mut Printer, node: &IntoClause) {
    node_range_var(str, node.rel.as_ref().unwrap(), DeparseNodeContext::None);

    if !node.col_names.is_empty() {
        str.word("(");
        node_column_list(str, &node.col_names);
        str.word(")");
    }
    str.word(" ");

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

fn node_select_stmt(str: &mut Printer, node: &SelectStmt) {}
