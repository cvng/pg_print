// Adapted from https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c.

use crate::algorithm::Printer;
use crate::INDENT;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::AConst;
use pg_query::protobuf::CollateClause;
use pg_query::protobuf::ColumnDef;
use pg_query::protobuf::ConstrType;
use pg_query::protobuf::Constraint;
use pg_query::protobuf::CreateStmt;
use pg_query::protobuf::DefElem;
use pg_query::protobuf::DefineStmt;
use pg_query::protobuf::Integer;
use pg_query::protobuf::ObjectType;
use pg_query::protobuf::RangeVar;
use pg_query::protobuf::RawStmt;
use pg_query::protobuf::TypeName;
use pg_query::Node;
use pg_query::NodeEnum;

const MONTH: i32 = 1;
const YEAR: i32 = 2;
const DAY: i32 = 3;
const HOUR: i32 = 10;
const MINUTE: i32 = 11;
const SECOND: i32 = 12;

const INTERVAL_FULL_RANGE: i32 = 0x7FFF;
const INTERVAL_FULL_PRECISION: i32 = 0xFFFF;

enum DeparseNodeContext {
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
    Constraint,
}

impl Printer {
    pub fn stmt(&mut self, stmt: &RawStmt) {
        match stmt.stmt.as_ref().unwrap().node.as_ref().unwrap() {
            NodeEnum::Alias(_) => todo!(),
            NodeEnum::RangeVar(_) => todo!(),
            NodeEnum::TableFunc(_) => todo!(),
            NodeEnum::Var(_) => todo!(),
            NodeEnum::Param(_) => todo!(),
            NodeEnum::Aggref(_) => todo!(),
            NodeEnum::GroupingFunc(_) => todo!(),
            NodeEnum::WindowFunc(_) => todo!(),
            NodeEnum::SubscriptingRef(_) => todo!(),
            NodeEnum::FuncExpr(_) => todo!(),
            NodeEnum::NamedArgExpr(_) => todo!(),
            NodeEnum::OpExpr(_) => todo!(),
            NodeEnum::DistinctExpr(_) => todo!(),
            NodeEnum::NullIfExpr(_) => todo!(),
            NodeEnum::ScalarArrayOpExpr(_) => todo!(),
            NodeEnum::BoolExpr(_) => todo!(),
            NodeEnum::SubLink(_) => todo!(),
            NodeEnum::SubPlan(_) => todo!(),
            NodeEnum::AlternativeSubPlan(_) => todo!(),
            NodeEnum::FieldSelect(_) => todo!(),
            NodeEnum::FieldStore(_) => todo!(),
            NodeEnum::RelabelType(_) => todo!(),
            NodeEnum::CoerceViaIo(_) => todo!(),
            NodeEnum::ArrayCoerceExpr(_) => todo!(),
            NodeEnum::ConvertRowtypeExpr(_) => todo!(),
            NodeEnum::CollateExpr(_) => todo!(),
            NodeEnum::CaseExpr(_) => todo!(),
            NodeEnum::CaseWhen(_) => todo!(),
            NodeEnum::CaseTestExpr(_) => todo!(),
            NodeEnum::ArrayExpr(_) => todo!(),
            NodeEnum::RowExpr(_) => todo!(),
            NodeEnum::RowCompareExpr(_) => todo!(),
            NodeEnum::CoalesceExpr(_) => todo!(),
            NodeEnum::MinMaxExpr(_) => todo!(),
            NodeEnum::SqlvalueFunction(_) => todo!(),
            NodeEnum::XmlExpr(_) => todo!(),
            NodeEnum::NullTest(_) => todo!(),
            NodeEnum::BooleanTest(_) => todo!(),
            NodeEnum::CoerceToDomain(_) => todo!(),
            NodeEnum::CoerceToDomainValue(_) => todo!(),
            NodeEnum::SetToDefault(_) => todo!(),
            NodeEnum::CurrentOfExpr(_) => todo!(),
            NodeEnum::NextValueExpr(_) => todo!(),
            NodeEnum::InferenceElem(_) => todo!(),
            NodeEnum::TargetEntry(_) => todo!(),
            NodeEnum::RangeTblRef(_) => todo!(),
            NodeEnum::JoinExpr(_) => todo!(),
            NodeEnum::FromExpr(_) => todo!(),
            NodeEnum::OnConflictExpr(_) => todo!(),
            NodeEnum::IntoClause(_) => todo!(),
            NodeEnum::MergeAction(_) => todo!(),
            NodeEnum::RawStmt(_) => todo!(),
            NodeEnum::Query(_) => todo!(),
            NodeEnum::InsertStmt(_) => todo!(),
            NodeEnum::DeleteStmt(_) => todo!(),
            NodeEnum::UpdateStmt(_) => todo!(),
            NodeEnum::MergeStmt(_) => todo!(),
            NodeEnum::SelectStmt(_) => todo!(),
            NodeEnum::ReturnStmt(_) => todo!(),
            NodeEnum::PlassignStmt(_) => todo!(),
            NodeEnum::AlterTableStmt(_) => todo!(),
            NodeEnum::AlterTableCmd(_) => todo!(),
            NodeEnum::AlterDomainStmt(_) => todo!(),
            NodeEnum::SetOperationStmt(_) => todo!(),
            NodeEnum::GrantStmt(_) => todo!(),
            NodeEnum::GrantRoleStmt(_) => todo!(),
            NodeEnum::AlterDefaultPrivilegesStmt(_) => todo!(),
            NodeEnum::ClosePortalStmt(_) => todo!(),
            NodeEnum::ClusterStmt(_) => todo!(),
            NodeEnum::CopyStmt(_) => todo!(),
            NodeEnum::CreateStmt(node) => node_create_stmt(self, node, false),
            NodeEnum::DefineStmt(node) => node_define_stmt(self, node),
            NodeEnum::DropStmt(_) => todo!(),
            NodeEnum::TruncateStmt(_) => todo!(),
            NodeEnum::CommentStmt(_) => todo!(),
            NodeEnum::FetchStmt(_) => todo!(),
            NodeEnum::IndexStmt(_) => todo!(),
            NodeEnum::CreateFunctionStmt(_) => todo!(),
            NodeEnum::AlterFunctionStmt(_) => todo!(),
            NodeEnum::DoStmt(_) => todo!(),
            NodeEnum::RenameStmt(_) => todo!(),
            NodeEnum::RuleStmt(_) => todo!(),
            NodeEnum::NotifyStmt(_) => todo!(),
            NodeEnum::ListenStmt(_) => todo!(),
            NodeEnum::UnlistenStmt(_) => todo!(),
            NodeEnum::TransactionStmt(_) => todo!(),
            NodeEnum::ViewStmt(_) => todo!(),
            NodeEnum::LoadStmt(_) => todo!(),
            NodeEnum::CreateDomainStmt(_) => todo!(),
            NodeEnum::CreatedbStmt(_) => todo!(),
            NodeEnum::DropdbStmt(_) => todo!(),
            NodeEnum::VacuumStmt(_) => todo!(),
            NodeEnum::ExplainStmt(_) => todo!(),
            NodeEnum::CreateTableAsStmt(_) => todo!(),
            NodeEnum::CreateSeqStmt(_) => todo!(),
            NodeEnum::AlterSeqStmt(_) => todo!(),
            NodeEnum::VariableSetStmt(_) => todo!(),
            NodeEnum::VariableShowStmt(_) => todo!(),
            NodeEnum::DiscardStmt(_) => todo!(),
            NodeEnum::CreateTrigStmt(_) => todo!(),
            NodeEnum::CreatePlangStmt(_) => todo!(),
            NodeEnum::CreateRoleStmt(_) => todo!(),
            NodeEnum::AlterRoleStmt(_) => todo!(),
            NodeEnum::DropRoleStmt(_) => todo!(),
            NodeEnum::LockStmt(_) => todo!(),
            NodeEnum::ConstraintsSetStmt(_) => todo!(),
            NodeEnum::ReindexStmt(_) => todo!(),
            NodeEnum::CheckPointStmt(_) => todo!(),
            NodeEnum::CreateSchemaStmt(_) => todo!(),
            NodeEnum::AlterDatabaseStmt(_) => todo!(),
            NodeEnum::AlterDatabaseRefreshCollStmt(_) => todo!(),
            NodeEnum::AlterDatabaseSetStmt(_) => todo!(),
            NodeEnum::AlterRoleSetStmt(_) => todo!(),
            NodeEnum::CreateConversionStmt(_) => todo!(),
            NodeEnum::CreateCastStmt(_) => todo!(),
            NodeEnum::CreateOpClassStmt(_) => todo!(),
            NodeEnum::CreateOpFamilyStmt(_) => todo!(),
            NodeEnum::AlterOpFamilyStmt(_) => todo!(),
            NodeEnum::PrepareStmt(_) => todo!(),
            NodeEnum::ExecuteStmt(_) => todo!(),
            NodeEnum::DeallocateStmt(_) => todo!(),
            NodeEnum::DeclareCursorStmt(_) => todo!(),
            NodeEnum::CreateTableSpaceStmt(_) => todo!(),
            NodeEnum::DropTableSpaceStmt(_) => todo!(),
            NodeEnum::AlterObjectDependsStmt(_) => todo!(),
            NodeEnum::AlterObjectSchemaStmt(_) => todo!(),
            NodeEnum::AlterOwnerStmt(_) => todo!(),
            NodeEnum::AlterOperatorStmt(_) => todo!(),
            NodeEnum::AlterTypeStmt(_) => todo!(),
            NodeEnum::DropOwnedStmt(_) => todo!(),
            NodeEnum::ReassignOwnedStmt(_) => todo!(),
            NodeEnum::CompositeTypeStmt(_) => todo!(),
            NodeEnum::CreateEnumStmt(_) => todo!(),
            NodeEnum::CreateRangeStmt(_) => todo!(),
            NodeEnum::AlterEnumStmt(_) => todo!(),
            NodeEnum::AlterTsdictionaryStmt(_) => todo!(),
            NodeEnum::AlterTsconfigurationStmt(_) => todo!(),
            NodeEnum::CreateFdwStmt(_) => todo!(),
            NodeEnum::AlterFdwStmt(_) => todo!(),
            NodeEnum::CreateForeignServerStmt(_) => todo!(),
            NodeEnum::AlterForeignServerStmt(_) => todo!(),
            NodeEnum::CreateUserMappingStmt(_) => todo!(),
            NodeEnum::AlterUserMappingStmt(_) => todo!(),
            NodeEnum::DropUserMappingStmt(_) => todo!(),
            NodeEnum::AlterTableSpaceOptionsStmt(_) => todo!(),
            NodeEnum::AlterTableMoveAllStmt(_) => todo!(),
            NodeEnum::SecLabelStmt(_) => todo!(),
            NodeEnum::CreateForeignTableStmt(_) => todo!(),
            NodeEnum::ImportForeignSchemaStmt(_) => todo!(),
            NodeEnum::CreateExtensionStmt(_) => todo!(),
            NodeEnum::AlterExtensionStmt(_) => todo!(),
            NodeEnum::AlterExtensionContentsStmt(_) => todo!(),
            NodeEnum::CreateEventTrigStmt(_) => todo!(),
            NodeEnum::AlterEventTrigStmt(_) => todo!(),
            NodeEnum::RefreshMatViewStmt(_) => todo!(),
            NodeEnum::ReplicaIdentityStmt(_) => todo!(),
            NodeEnum::AlterSystemStmt(_) => todo!(),
            NodeEnum::CreatePolicyStmt(_) => todo!(),
            NodeEnum::AlterPolicyStmt(_) => todo!(),
            NodeEnum::CreateTransformStmt(_) => todo!(),
            NodeEnum::CreateAmStmt(_) => todo!(),
            NodeEnum::CreatePublicationStmt(_) => todo!(),
            NodeEnum::AlterPublicationStmt(_) => todo!(),
            NodeEnum::CreateSubscriptionStmt(_) => todo!(),
            NodeEnum::AlterSubscriptionStmt(_) => todo!(),
            NodeEnum::DropSubscriptionStmt(_) => todo!(),
            NodeEnum::CreateStatsStmt(_) => todo!(),
            NodeEnum::AlterCollationStmt(_) => todo!(),
            NodeEnum::CallStmt(_) => todo!(),
            NodeEnum::AlterStatsStmt(_) => todo!(),
            NodeEnum::AExpr(_) => todo!(),
            NodeEnum::ColumnRef(_) => todo!(),
            NodeEnum::ParamRef(_) => todo!(),
            NodeEnum::FuncCall(_) => todo!(),
            NodeEnum::AStar(_) => todo!(),
            NodeEnum::AIndices(_) => todo!(),
            NodeEnum::AIndirection(_) => todo!(),
            NodeEnum::AArrayExpr(_) => todo!(),
            NodeEnum::ResTarget(_) => todo!(),
            NodeEnum::MultiAssignRef(_) => todo!(),
            NodeEnum::TypeCast(_) => todo!(),
            NodeEnum::CollateClause(_) => todo!(),
            NodeEnum::SortBy(_) => todo!(),
            NodeEnum::WindowDef(_) => todo!(),
            NodeEnum::RangeSubselect(_) => todo!(),
            NodeEnum::RangeFunction(_) => todo!(),
            NodeEnum::RangeTableSample(_) => todo!(),
            NodeEnum::RangeTableFunc(_) => todo!(),
            NodeEnum::RangeTableFuncCol(_) => todo!(),
            NodeEnum::TypeName(_) => todo!(),
            NodeEnum::ColumnDef(_) => todo!(),
            NodeEnum::IndexElem(_) => todo!(),
            NodeEnum::StatsElem(_) => todo!(),
            NodeEnum::Constraint(_) => todo!(),
            NodeEnum::DefElem(_) => todo!(),
            NodeEnum::RangeTblEntry(_) => todo!(),
            NodeEnum::RangeTblFunction(_) => todo!(),
            NodeEnum::TableSampleClause(_) => todo!(),
            NodeEnum::WithCheckOption(_) => todo!(),
            NodeEnum::SortGroupClause(_) => todo!(),
            NodeEnum::GroupingSet(_) => todo!(),
            NodeEnum::WindowClause(_) => todo!(),
            NodeEnum::ObjectWithArgs(_) => todo!(),
            NodeEnum::AccessPriv(_) => todo!(),
            NodeEnum::CreateOpClassItem(_) => todo!(),
            NodeEnum::TableLikeClause(_) => todo!(),
            NodeEnum::FunctionParameter(_) => todo!(),
            NodeEnum::LockingClause(_) => todo!(),
            NodeEnum::RowMarkClause(_) => todo!(),
            NodeEnum::XmlSerialize(_) => todo!(),
            NodeEnum::WithClause(_) => todo!(),
            NodeEnum::InferClause(_) => todo!(),
            NodeEnum::OnConflictClause(_) => todo!(),
            NodeEnum::CtesearchClause(_) => todo!(),
            NodeEnum::CtecycleClause(_) => todo!(),
            NodeEnum::CommonTableExpr(_) => todo!(),
            NodeEnum::MergeWhenClause(_) => todo!(),
            NodeEnum::RoleSpec(_) => todo!(),
            NodeEnum::TriggerTransition(_) => todo!(),
            NodeEnum::PartitionElem(_) => todo!(),
            NodeEnum::PartitionSpec(_) => todo!(),
            NodeEnum::PartitionBoundSpec(_) => todo!(),
            NodeEnum::PartitionRangeDatum(_) => todo!(),
            NodeEnum::PartitionCmd(_) => todo!(),
            NodeEnum::VacuumRelation(_) => todo!(),
            NodeEnum::PublicationObjSpec(_) => todo!(),
            NodeEnum::PublicationTable(_) => todo!(),
            NodeEnum::InlineCodeBlock(_) => todo!(),
            NodeEnum::CallContext(_) => todo!(),
            NodeEnum::Integer(_) => todo!(),
            NodeEnum::Float(_) => todo!(),
            NodeEnum::Boolean(_) => todo!(),
            NodeEnum::String(_) => todo!(),
            NodeEnum::BitString(_) => todo!(),
            NodeEnum::List(_) => todo!(),
            NodeEnum::IntList(_) => todo!(),
            NodeEnum::OidList(_) => todo!(),
            NodeEnum::AConst(_) => todo!(),
        }
    }
}

fn node_create_stmt(str: &mut Printer, node: &CreateStmt, is_foreign_table: bool) {
    str.cbox(INDENT);
    str.keyword("create ");

    if is_foreign_table {
        str.keyword("foreign ");
    }

    // TODO: node_opt_temp(str, &node.relation.unwrap().relpersistence);

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

    if node.of_typename.is_some() {
        str.keyword("of ");
        node_type_name(str, node.of_typename.as_ref().unwrap());
        str.space();
    }

    if !node.table_elts.is_empty() {
        str.word("(");
        str.hardbreak_if_nonempty();
        for (i, elt) in node.table_elts.iter().enumerate() {
            node_table_element(str, elt);
            str.comma(i >= node.table_elts.len() - 1);
            str.hardbreak();
        }
        str.offset(-INDENT);
        str.end();
        str.word(")");
    }

    str.hardbreak();
}

fn node_range_var(str: &mut Printer, node: &RangeVar, _ctx: DeparseNodeContext) {
    str.ident(node.relname.clone());
}

fn node_define_stmt(str: &mut Printer, node: &DefineStmt) {
    str.cbox(0);
    str.keyword("create ");

    if node.replace {
        str.keyword("or replace ");
    }

    match node.kind() {
        ObjectType::ObjectAggregate => str.keyword("aggregate "),
        ObjectType::ObjectOperator => str.keyword("operator "),
        ObjectType::ObjectType => str.keyword("type "),
        ObjectType::ObjectTsparser => str.keyword("text search parser "),
        ObjectType::ObjectTsdictionary => str.keyword("text seach dictionary "),
        ObjectType::ObjectTstemplate => str.keyword("text search template "),
        ObjectType::ObjectTsconfiguration => str.keyword("text search configuration "),
        ObjectType::ObjectCollation => str.keyword("collation "),
        _ => unreachable!(),
    };

    if node.if_not_exists {
        str.keyword("if not exists ");
    }

    match node.kind() {
        ObjectType::ObjectAggregate => todo!(),
        ObjectType::ObjectOperator => todo!(),
        ObjectType::ObjectType
        | ObjectType::ObjectTsparser
        | ObjectType::ObjectTsdictionary
        | ObjectType::ObjectTstemplate
        | ObjectType::ObjectTsconfiguration
        | ObjectType::ObjectCollation => node_any_name(str, &node.defnames),
        _ => unreachable!(),
    }
    str.space();

    if !node.oldstyle && matches!(node.kind(), ObjectType::ObjectAggregate) {
        todo!();
        str.space();
    }

    if (matches!(node.kind(), ObjectType::ObjectCollation)
        && node.definition.len() == 1
        && matches!(
            node.definition.first().unwrap().node.as_ref().unwrap(),
            NodeEnum::DefElem(node) if node.defname == "from",
        ))
    {
        str.keyword("from ");
        todo!();
    } else if (!node.definition.is_empty()) {
        todo!()
    }

    str.end();
}

fn node_column_def(str: &mut Printer, node: &ColumnDef) {
    str.ident(node.colname.clone());

    if let Some(type_name) = &node.type_name {
        str.nbsp();
        node_type_name(str, type_name);
    }

    if let Some(raw_default) = &node.raw_default {
        str.nbsp();
        str.word("using ");
        node_expr(str, raw_default);
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

    if node.names.len() == 2 && str_val(node.names.first().unwrap()) == "pg_catalog" {
        let name = str_val(node.names.last().unwrap());
        match name.as_str() {
            "int4" => str.word("int"),
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
            "interval" if node.typmods.is_empty() => str.word("interval"),
            "interval" if !node.typmods.is_empty() => {
                str.word("interval");
                node_interval_typmods(str, node);

                skip_typmods = true;
            }
            _ => {
                str.word("pg_catalog.");
                str.word(name);
            }
        }
    } else {
        node_any_name(str, &node.names);
    }
}

fn node_signed_iconst(str: &mut Printer, node: &Node) {
    str.word(format!("{}", int_val(node)));
}

fn node_interval_typmods(str: &mut Printer, node: &TypeName) {
    let fields = node
        .typmods
        .first()
        .map(a_const_ival)
        .map(|ival| Some(NodeEnum::Integer(Integer { ival })))
        .map(|node| Node { node })
        .as_ref()
        .map(int_val)
        .unwrap();

    // See https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c#L3784.
    match fields {
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
            .map(a_const_ival)
            .map(|ival| Some(NodeEnum::Integer(Integer { ival })))
            .map(|node| Node { node })
            .as_ref()
            .map(int_val)
            .unwrap();

        if precision != INTERVAL_FULL_PRECISION {
            str.word(format!("{}", precision));
        }
    }
}

fn node_expr(str: &mut Printer, node: &Node) {
    todo!()
}

fn node_create_generic_options(str: &mut Printer, list: &[Node]) {
    todo!()
}

fn node_constraint(str: &mut Printer, node: &Constraint) {
    if !node.conname.is_empty() {
        str.keyword("constraint ");
        str.ident(node.conname.clone());
        str.space();
    }

    match node.contype() {
        ConstrType::ConstrPrimary => str.keyword("primary key"),
        _ => todo!(),
    }
}

fn node_opt_with(str: &mut Printer, list: &[Node]) {
    if !list.is_empty() {
        str.keyword("with ");
        node_rel_options(str, list);
        str.space();
    }
}

fn node_rel_options(str: &mut Printer, list: &[Node]) {
    todo!()
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

fn node_any_name(str: &mut Printer, list: &[Node]) {
    for (i, part) in list.iter().enumerate() {
        if i > 0 {
            str.word(".");
        }
        str.ident(str_val(part));
    }
}

fn str_val(node: &Node) -> String {
    match node.node.as_ref().unwrap() {
        NodeEnum::String(node) => node.sval.clone(),
        _ => unreachable!(),
    }
}

fn int_val(node: &Node) -> i32 {
    match node.node.as_ref().unwrap() {
        NodeEnum::Integer(node) => node.ival,
        _ => unreachable!(),
    }
}

fn a_const_ival(node: &Node) -> i32 {
    match node.node.as_ref().unwrap() {
        NodeEnum::AConst(AConst {
            val: Some(Val::Ival(Integer { ival, .. })),
            ..
        }) => *ival,
        _ => unreachable!(),
    }
}
