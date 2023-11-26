// Adapted from https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c.

use crate::algorithm::Printer;
use pg_query::protobuf::CollateClause;
use pg_query::protobuf::ColumnDef;
use pg_query::protobuf::ConstrType;
use pg_query::protobuf::Constraint;
use pg_query::protobuf::CreateStmt;
use pg_query::protobuf::DefElem;
use pg_query::protobuf::DefineStmt;
use pg_query::protobuf::ObjectType;
use pg_query::protobuf::RangeVar;
use pg_query::protobuf::RawStmt;
use pg_query::protobuf::TypeName;
use pg_query::Node;
use pg_query::NodeEnum;

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
    str.cbox(0);

    str.word("create ");

    if is_foreign_table {
        str.word("foreign ");
    }

    // TODO: node_opt_temp(str, &node.relation.unwrap().relpersistence);

    str.word("table ");

    if node.if_not_exists {
        str.word("if not exists ");
    }

    node_range_var(
        str,
        node.relation.as_ref().unwrap(),
        DeparseNodeContext::None,
    );
    str.nbsp();

    if node.of_typename.is_some() {
        str.word("of ");
        node_type_name(str, node.of_typename.as_ref().unwrap());
        str.space();
    }

    if !node.table_elts.is_empty() {
        str.word("(");
        str.hardbreak();
        for (i, elt) in node.table_elts.iter().enumerate() {
            if i > 0 {
                str.trailing_comma(i == node.table_elts.len());
            }
            node_table_element(str, elt);
        }
        str.word(")");
    }

    str.end();
}

fn node_range_var(str: &mut Printer, node: &RangeVar, _ctx: DeparseNodeContext) {
    str.ident(node.relname.clone());
}

fn node_define_stmt(str: &mut Printer, node: &DefineStmt) {
    str.cbox(0);

    str.word("create ");

    if node.replace {
        str.word("or replace ");
    }

    match node.kind() {
        ObjectType::ObjectAggregate => str.word("aggregate "),
        ObjectType::ObjectOperator => str.word("operator "),
        ObjectType::ObjectType => str.word("type "),
        ObjectType::ObjectTsparser => str.word("text search parser "),
        ObjectType::ObjectTsdictionary => str.word("text seach dictionary "),
        ObjectType::ObjectTstemplate => str.word("text search template "),
        ObjectType::ObjectTsconfiguration => str.word("text search configuration "),
        ObjectType::ObjectCollation => str.word("collation "),
        _ => unreachable!(),
    };

    if node.if_not_exists {
        str.word("if not exists ");
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
            NodeEnum::DefElem(node) if node.defname.eq("from"),
        ))
    {
        str.word("from ");
        todo!();
    } else if (!node.definition.is_empty()) {
        todo!()
    }

    str.end();
}

fn node_column_def(str: &mut Printer, node: &ColumnDef) {
    if !node.colname.is_empty() {
        str.ident(node.colname.clone());
        str.nbsp();
    }

    if (node.type_name.is_some()) {
        node_type_name(str, node.type_name.as_ref().unwrap());
        str.nbsp();
    }

    if (node.raw_default.is_some()) {
        str.word("using ");
        node_expr(str, node.raw_default.as_ref().unwrap());
        str.space();
    }

    if !node.fdwoptions.is_empty() {
        node_create_generic_options(str, &node.fdwoptions);
        str.space();
    }

    for constraint in node.constraints.iter() {
        match constraint.node.as_ref().unwrap() {
            NodeEnum::Constraint(constraint) => {
                node_constraint(str, constraint);
                str.space();
            }
            _ => unreachable!(),
        }
    }

    if (node.coll_clause.is_some()) {
        node_collate_clause(str, node.coll_clause.as_ref().unwrap());
    }
}

fn node_type_name(str: &mut Printer, node: &TypeName) {
    if node.names.len() == 2 && str_val(node.names.first().unwrap()).eq("pg_catalog") {
        match str_val(node.names.last().unwrap()).as_str() {
            "int4" => str.word("int"),
            _ => todo!(),
        }
    } else {
        node_any_name(str, &node.names);
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
        str.word("constraint ");
        str.ident(node.conname.clone());
        str.space();
    }

    match node.contype() {
        ConstrType::ConstrPrimary => str.word("primary key "),
        _ => todo!(),
    }
}

fn node_opt_with(str: &mut Printer, list: &[Node]) {
    if !list.is_empty() {
        str.word("with ");
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
        NodeEnum::Constraint(_) => todo!(),
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
