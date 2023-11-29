use crate::algorithm::Printer;
use crate::INDENT;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::AConst;
use pg_query::protobuf::AExpr;
use pg_query::protobuf::AExprKind;
use pg_query::protobuf::AStar;
use pg_query::protobuf::CollateClause;
use pg_query::protobuf::ColumnDef;
use pg_query::protobuf::ColumnRef;
use pg_query::protobuf::ConstrType;
use pg_query::protobuf::Constraint;
use pg_query::protobuf::CreateStmt;
use pg_query::protobuf::DefElem;
use pg_query::protobuf::DefineStmt;
use pg_query::protobuf::Integer;
use pg_query::protobuf::ObjectType;
use pg_query::protobuf::OnCommitAction;
use pg_query::protobuf::ParamRef;
use pg_query::protobuf::PartitionBoundSpec;
use pg_query::protobuf::RangeVar;
use pg_query::protobuf::RawStmt;
use pg_query::protobuf::TypeName;
use pg_query::Node;
use pg_query::NodeEnum;
use std::ops::Deref;

const MONTH: i32 = 1;
const YEAR: i32 = 2;
const DAY: i32 = 3;
const HOUR: i32 = 10;
const MINUTE: i32 = 11;
const SECOND: i32 = 12;

const INTERVAL_FULL_RANGE: i32 = 0x7FFF;
const INTERVAL_FULL_PRECISION: i32 = 0xFFFF;

const RELPERSISTENCE_TEMP: char = 't';
const RELPERSISTENCE_UNLOGGED: char = 'u';
const RELPERSISTENCE_PERMANENT: char = 'p';

const PARTITION_STRATEGY_HASH: char = 'h';
const PARTITION_STRATEGY_LIST: char = 'l';
const PARTITION_STRATEGY_RANGE: char = 'r';

const ESCAPE_STRING_SYNTAX: char = 'E';

pub enum DeparseNodeContext {
    None,
    // Parent node type (and sometimes field).
    InsertRelation,
    InsertOnConflict,
    Update,
    Returning,
    AExpr,
    Xmlattributes,
    Xmlnamespaces,
    CreateType,
    AlterType,
    SetStatement,
    // Identifier vs constant context.
    Identifier,
    Constant,
}

// See https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c#L53.
fn deparse_string_literal(str: &mut Printer, val: &str) {
    if val.contains('\\') {
        str.word(ESCAPE_STRING_SYNTAX.to_string());
    }

    str.word('\''.to_string());

    for c in val.chars() {
        if c == '\'' || c == '\\' {
            str.word(c.to_string());
        }

        str.word(c.to_string());
    }

    str.word('\''.to_string());
}

pub fn node_create_stmt(str: &mut Printer, node: &CreateStmt, is_foreign_table: bool) {
    str.cbox(INDENT);
    str.keyword("create ");

    if is_foreign_table {
        str.keyword("foreign ");
    }

    node_opt_temp(str, &node.relation.as_ref().unwrap().relpersistence);

    str.keyword("table ");

    if node.if_not_exists {
        str.keyword("if not exists ");
    }

    node_range_var(
        str,
        node.relation.as_ref().unwrap(),
        DeparseNodeContext::None,
    );
    str.nbsp();

    if let Some(of_typename) = &node.of_typename {
        str.keyword("of ");
        node_type_name(str, of_typename);
        str.space();
    }

    if node.partbound.is_some() {
        str.keyword("partition of ");
        node_range_var(
            str,
            node.inh_relations
                .first()
                .and_then(|node| match node.node.as_ref().unwrap() {
                    NodeEnum::RangeVar(node) => Some(node),
                    _ => None,
                })
                .unwrap(),
            DeparseNodeContext::None,
        );
        str.word(" ");
    }

    if !node.table_elts.is_empty() {
        str.word("(");
        str.hardbreak_if_nonempty();
        for (i, elt) in node.table_elts.iter().enumerate() {
            node_table_element(str, elt);
            if i < node.table_elts.len() - 1 {
                str.word(",");
            }
            str.hardbreak();
        }
        str.offset(-INDENT);
        str.end();
        str.word(")");
    } else if node.partbound.is_none() && node.of_typename.is_none() {
        str.word("()");
    };

    if let Some(partbound) = &node.partbound {
        node_partition_bound_spec(str, partbound);
        str.word(" ");
    } else {
        node_opt_inherit(str, &node.inh_relations);
    }

    node_opt_with(str, &node.options);

    node_on_commit_action(str, &node.oncommit());

    if !node.tablespacename.is_empty() {
        str.keyword("tablespace ");
        str.ident(node.tablespacename.clone());
    }

    str.hardbreak();
}

pub fn node_on_commit_action(str: &mut Printer, node: &OnCommitAction) {
    match node {
        OnCommitAction::Undefined => {}
        OnCommitAction::OncommitNoop => {}
        OnCommitAction::OncommitPreserveRows => str.keyword(" on commit preserve rows"),
        OnCommitAction::OncommitDeleteRows => str.keyword(" on commit delete rows"),
        OnCommitAction::OncommitDrop => str.keyword(" on commit drop"),
    }
}

fn node_opt_inherit(str: &mut Printer, list: &[Node]) {
    if !list.is_empty() {
        todo!("{:?}", list)
    }
}

fn node_partition_bound_spec(str: &mut Printer, node: &PartitionBoundSpec) {
    if node.is_default {
        str.keyword("default");
        return;
    }

    str.keyword(" for values ");

    match node.strategy.chars().next().unwrap() {
        PARTITION_STRATEGY_HASH => {
            str.keyword(format!(
                "with (modulus {}, remainder {})",
                node.modulus, node.remainder
            ));
        }
        PARTITION_STRATEGY_LIST => {
            str.keyword("in (");
            node_expr_list(str, &node.listdatums);
            str.word(")");
        }
        PARTITION_STRATEGY_RANGE => {
            str.keyword("from (");
            node_expr_list(str, &node.lowerdatums);
            str.keyword(") to (");
            node_expr_list(str, &node.upperdatums);
            str.word(")");
        }
        _ => unreachable!(),
    }
}

pub fn node_expr_list(str: &mut Printer, list: &[Node]) {
    for (i, expr) in list.iter().enumerate() {
        node_expr(str, Some(expr));
        str.comma(i >= list.len() - 1);
    }
}

pub fn node_opt_temp(str: &mut Printer, persistence: &str) {
    match persistence.chars().next().unwrap() {
        RELPERSISTENCE_TEMP => str.keyword("temporary "),
        RELPERSISTENCE_UNLOGGED => str.keyword("unlogged "),
        RELPERSISTENCE_PERMANENT => {}
        _ => unreachable!(),
    }
}

pub fn node_range_var(str: &mut Printer, node: &RangeVar, context: DeparseNodeContext) {
    str.ident(node.relname.clone());
}

fn node_column_def(str: &mut Printer, node: &ColumnDef) {
    if !node.colname.is_empty() {
        str.ident(node.colname.clone());
    }

    if let Some(type_name) = &node.type_name {
        str.nbsp();
        node_type_name(str, type_name);
    }

    if node.raw_default.is_some() {
        str.nbsp();
        str.word("using ");
        node_expr(str, node.raw_default.as_deref());
    }

    if !node.fdwoptions.is_empty() {
        str.nbsp();
        node_create_generic_options(str, &node.fdwoptions);
    }

    for constraint in node.constraints.iter() {
        match constraint.node.as_ref().unwrap() {
            NodeEnum::Constraint(constraint) => {
                str.nbsp();
                node_constraint(str, constraint);
            }
            _ => unreachable!(),
        }
    }

    if (node.coll_clause.is_some()) {
        node_collate_clause(str, node.coll_clause.as_ref().unwrap());
    }
}

fn node_type_name(str: &mut Printer, node: &TypeName) {
    let mut skip_typmods = false;

    if node.setof {
        str.keyword("setof ");
    }

    if node.names.len() == 2 && str_val(node.names.first().unwrap()).unwrap() == "pg_catalog" {
        let name = str_val(node.names.last().unwrap()).unwrap();

        match name.as_str() {
            "bpchar" => str.word("char"),
            "varchar" => str.word("varchar"),
            "numeric" => str.word("numeric"),
            "bool" => str.word("boolean"),
            "int2" => str.word("smallint"),
            "int4" => str.word("int"),
            "int8" => str.word("bigint"),
            "real" | "float4" => str.word("real"),
            "float8" => str.word("double precision"),
            "time" => str.word("time"),
            "timetz" => {
                str.word("time ");
                if !node.typmods.is_empty() {
                    str.word("(");
                    for (i, typmod) in node.typmods.iter().enumerate() {
                        node_signed_iconst(str, typmod);
                        str.comma(i >= node.typmods.len() - 1);
                    }
                    str.word(") ");
                }
                str.word("with time zone");
                skip_typmods = true;
            }
            "timestamp" => str.word("timestamp"),
            "timestamptz" => {
                str.word("timestamp ");
                if !node.typmods.is_empty() {
                    str.word("(");
                    for (i, typmod) in node.typmods.iter().enumerate() {
                        node_signed_iconst(str, typmod);
                        str.comma(i >= node.typmods.len() - 1);
                    }
                    str.word(") ");
                }
                str.word("with time zone");
                skip_typmods = true;
            }
            "interval" => {
                str.word("interval");

                if !node.typmods.is_empty() {
                    node_interval_typmods(str, node);
                    skip_typmods = true;
                }
            }
            _ => {
                str.word("pg_catalog.");
                str.word(name);
            }
        }
    } else {
        node_any_name(str, &node.names);
    }

    if !node.typmods.is_empty() && !skip_typmods {
        str.word("(");
        for (i, typmod) in node.typmods.iter().enumerate() {
            match typmod.node.as_ref().unwrap() {
                NodeEnum::AConst(node) => node_a_const(str, node),
                NodeEnum::ParamRef(node) => node_param_ref(str, node),
                NodeEnum::ColumnRef(node) => node_column_ref(str, node),
                _ => unreachable!(),
            }
            str.comma(i >= node.typmods.len() - 1);
        }
        str.word(")");
    }
}

fn node_value(str: &mut Printer, node: Option<&Val>, context: DeparseNodeContext) {
    let Some(val) = node else {
        str.keyword("null");
        return;
    };

    match val {
        Val::Ival(_) | Val::Fval(_) => node_numeric_only(str, val),
        Val::Boolval(val) => str.word(if val.boolval { "true" } else { "false" }),
        Val::Sval(val) => match context {
            DeparseNodeContext::Identifier => str.ident(val.sval.clone()),
            DeparseNodeContext::Constant => deparse_string_literal(str, &val.sval),
            _ => str.word(val.sval.clone()),
        },
        Val::Bsval(val) => match val.bsval.chars().next().unwrap() {
            'x' => {
                str.word("x");
                deparse_string_literal(str, &val.bsval[1..])
            }
            'b' => {
                str.word("b");
                deparse_string_literal(str, &val.bsval[1..])
            }
            _ => unreachable!(),
        },
    }
}

fn node_numeric_only(str: &mut Printer, val: &Val) {
    match val {
        Val::Ival(val) => str.word(format!("{}", val.ival)),
        Val::Fval(val) => str.word(val.fval.clone()),
        _ => unreachable!(),
    }
}

fn node_a_const(str: &mut Printer, node: &AConst) {
    node_value(str, node.val.as_ref(), DeparseNodeContext::Constant);
}

fn node_a_expr(str: &mut Printer, node: &AExpr, context: DeparseNodeContext) {
    let need_lexpr_parens = false;
    let need_rexpr_parens = false;

    match node.kind() {
        AExprKind::Undefined => todo!(),
        AExprKind::AexprOp => {
            let need_outer_parens = matches!(context, DeparseNodeContext::AExpr);

            if need_outer_parens {
                str.word("(");
            }

            if node.lexpr.is_some() {
                if need_lexpr_parens {
                    str.word("(");
                }

                node_expr(str, node.lexpr.as_deref());

                if need_lexpr_parens {
                    str.word(")");
                }

                str.nbsp();
            }

            node_qual_op(str, &node.name);

            if node.rexpr.is_some() {
                str.nbsp();

                if need_rexpr_parens {
                    str.word("(");
                }

                node_expr(str, node.rexpr.as_deref());

                if need_rexpr_parens {
                    str.word(")");
                }
            }
        }
        AExprKind::AexprOpAny => todo!(),
        AExprKind::AexprOpAll => todo!(),
        AExprKind::AexprDistinct => todo!(),
        AExprKind::AexprNotDistinct => todo!(),
        AExprKind::AexprNullif => todo!(),
        AExprKind::AexprIn => todo!(),
        AExprKind::AexprLike => todo!(),
        AExprKind::AexprIlike => todo!(),
        AExprKind::AexprSimilar => todo!(),
        AExprKind::AexprBetween => todo!(),
        AExprKind::AexprNotBetween => todo!(),
        AExprKind::AexprBetweenSym => todo!(),
        AExprKind::AexprNotBetweenSym => todo!(),
    }
}

fn node_qual_op(str: &mut Printer, list: &[Node]) {
    if list.len() == 1 && is_op(str_val(list.first().unwrap())) {
        str.word(str_val(list.first().unwrap()).unwrap());
    } else {
        str.word("operator(");
        node_any_operator(str, list);
        str.word(")");
    }
}

fn node_any_operator(str: &mut Printer, list: &[Node]) {
    match list.len() {
        2 => {
            str.ident(str_val(list.first().unwrap()).unwrap());
            str.word(".");
            str.word(str_val(list.last().unwrap()).unwrap());
        }
        1 => str.ident(str_val(list.last().unwrap()).unwrap()),
        _ => unreachable!(),
    }
}

fn node_param_ref(str: &mut Printer, node: &ParamRef) {
    todo!()
}

pub fn node_column_ref(str: &mut Printer, node: &ColumnRef) {
    if let NodeEnum::AStar(node) = node.fields.first().unwrap().node.as_ref().unwrap() {
        node_a_star(str, node);
    } else if let NodeEnum::String(node) = node.fields.first().unwrap().node.as_ref().unwrap() {
        node_col_label(str, &node.sval);
    }

    node_opt_indirection(str, &node.fields, 1);
}

fn node_a_star(str: &mut Printer, node: &AStar) {
    str.word("*");
}

fn node_col_label(str: &mut Printer, node: &str) {
    str.ident(node.to_owned());
}

fn node_opt_indirection(str: &mut Printer, list: &[Node], offset: usize) {
    for (i, item) in list.iter().enumerate().skip(offset) {}
}

fn node_signed_iconst(str: &mut Printer, node: &Node) {
    str.word(format!("{}", int_val(node).unwrap()));
}

// See https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c#L3774.
fn node_interval_typmods(str: &mut Printer, node: &TypeName) {
    let interval_fields = node
        .typmods
        .first()
        .and_then(a_const_int_val)
        .map(|ival| Some(NodeEnum::Integer(Integer { ival })))
        .map(|node| Node { node })
        .as_ref()
        .map(int_val)
        .unwrap()
        .unwrap();

    match interval_fields {
        x if x == 1 << YEAR => str.word(" year"),
        x if x == 1 << MONTH => str.word(" month"),
        x if x == 1 << DAY => str.word(" day"),
        x if x == 1 << HOUR => str.word(" hour"),
        x if x == 1 << MINUTE => str.word(" minute"),
        x if x == 1 << SECOND => str.word(" second"),
        x if x == 1 << YEAR | 1 << MONTH => str.word(" year to month"),
        x if x == 1 << DAY | 1 << HOUR => str.word(" day to hour"),
        x if x == 1 << DAY | 1 << HOUR | 1 << MINUTE => str.word(" day to minute"),
        x if x == 1 << DAY | 1 << HOUR | 1 << MINUTE | 1 << SECOND => str.word(" day to second"),
        x if x == 1 << HOUR | 1 << MINUTE => str.word(" hour to minute"),
        x if x == 1 << HOUR | 1 << MINUTE | 1 << SECOND => str.word(" hour to second"),
        x if x == 1 << MINUTE | 1 << SECOND => str.word(" minute to second"),
        INTERVAL_FULL_RANGE => {}
        _ => unreachable!(),
    };

    if node.typmods.len() == 2 {
        let precision = node
            .typmods
            .last()
            .and_then(a_const_int_val)
            .map(|ival| Some(NodeEnum::Integer(Integer { ival })))
            .map(|node| Node { node })
            .as_ref()
            .map(int_val)
            .unwrap()
            .unwrap();

        if precision != INTERVAL_FULL_PRECISION {
            str.word(format!(" ({})", precision));
        }
    }
}

pub fn node_expr(str: &mut Printer, node: Option<&Node>) {
    let Some(node) = node else {
        return;
    };

    match node.node.as_ref().unwrap() {
        NodeEnum::AConst(node) => node_a_const(str, node),
        NodeEnum::AExpr(node) => node_a_expr(str, node, DeparseNodeContext::None),
        node => todo!("{:?}", node),
    }
}

fn node_create_generic_options(str: &mut Printer, list: &[Node]) {
    todo!()
}

fn node_constraint(str: &mut Printer, node: &Constraint) {
    if !node.conname.is_empty() {
        str.keyword("constraint ");
        str.ident(node.conname.clone());
        str.nbsp();
    }

    match node.contype() {
        ConstrType::ConstrDefault => {
            str.keyword("default ");
            node_expr(str, node.raw_expr.as_deref());
        }
        ConstrType::ConstrPrimary => str.keyword("primary key"),
        ConstrType::ConstrUnique => str.keyword("unique"),
        _ => todo!("{:?}", node.contype()),
    }

    if !node.keys.is_empty() {
        str.nbsp();
        str.word("(");
        node_column_list(str, &node.keys);
        str.word(")");
    }

    match node.contype() {
        ConstrType::ConstrPrimary | ConstrType::ConstrUnique | ConstrType::ConstrExclusion => {
            node_opt_with(str, &node.options)
        }
        _ => {}
    }

    if !node.indexspace.is_empty() {
        str.keyword("using index tablespace ");
        str.ident(node.indexspace.clone());
    }
}

pub fn node_column_list(str: &mut Printer, list: &[Node]) {
    for (i, column) in list.iter().enumerate() {
        str.ident(str_val(column).unwrap());
        if i < list.len() - 1 {
            str.word(", ");
        }
    }
}

pub fn node_opt_with(str: &mut Printer, list: &[Node]) {
    if !list.is_empty() {
        str.keyword(" with ");
        node_rel_options(str, list);
        str.nbsp();
    }
}

fn node_rel_options(str: &mut Printer, list: &[Node]) {
    str.word("(");

    for (i, option) in list.iter().enumerate() {
        match option.node.as_ref().unwrap() {
            NodeEnum::DefElem(node) => {
                if !node.defnamespace.is_empty() {
                    str.ident(node.defnamespace.clone());
                    str.word(".");
                }
                str.ident(node.defname.clone());
                if let Some(arg) = &node.arg {
                    str.word(" = ");
                    node_def_arg(str, arg, false);
                }
            }
            _ => unreachable!(),
        }

        str.comma(i >= list.len() - 1);
    }

    str.word(")");
}

fn node_def_arg(str: &mut Printer, node: &Node, is_operator_def_arg: bool) {
    match node.node.as_ref().unwrap() {
        NodeEnum::Integer(ref val) => {
            node_value(str, Some(&Val::Ival(val.clone())), DeparseNodeContext::None)
        }
        _ => todo!(),
    }
}

fn node_collate_clause(str: &mut Printer, node: &CollateClause) {
    todo!()
}

fn node_table_element(str: &mut Printer, node: &Node) {
    match node.node.as_ref().unwrap() {
        NodeEnum::ColumnDef(node) => node_column_def(str, node),
        NodeEnum::Constraint(node) => node_constraint(str, node),
        NodeEnum::IndexElem(_) => todo!(),
        NodeEnum::DefElem(_) => todo!(),
        _ => unreachable!(),
    }
}

pub fn node_any_name(str: &mut Printer, list: &[Node]) {
    for (i, part) in list.iter().enumerate() {
        if i > 0 {
            str.word(".");
        }
        str.ident(str_val(part).unwrap());
    }
}

fn str_val(node: &Node) -> Option<String> {
    match node.node.as_ref().unwrap() {
        NodeEnum::String(val) => Some(val.sval.clone()),
        _ => None,
    }
}

fn int_val(node: &Node) -> Option<i32> {
    match node.node.as_ref().unwrap() {
        NodeEnum::Integer(val) => Some(val.ival),
        _ => None,
    }
}

fn a_const_int_val(node: &Node) -> Option<i32> {
    match node.node.as_ref().unwrap() {
        NodeEnum::AConst(AConst {
            val: Some(Val::Ival(val)),
            ..
        }) => Some(val.ival),
        _ => None,
    }
}
