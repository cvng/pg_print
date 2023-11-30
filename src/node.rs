// Adapted from https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c.

use crate::fmt;
use crate::utils;
use pg_query::Node;
use pg_query::NodeEnum;

impl fmt::Print for Node {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        match self.node.as_ref().unwrap() {
            NodeEnum::Alias(node) => todo!("{:?}", node),
            NodeEnum::RangeVar(node) => node.print(p),
            NodeEnum::TableFunc(node) => todo!("{:?}", node),
            NodeEnum::Var(node) => todo!("{:?}", node),
            NodeEnum::Param(node) => todo!("{:?}", node),
            NodeEnum::Aggref(node) => todo!("{:?}", node),
            NodeEnum::GroupingFunc(node) => todo!("{:?}", node),
            NodeEnum::WindowFunc(node) => todo!("{:?}", node),
            NodeEnum::SubscriptingRef(node) => todo!("{:?}", node),
            NodeEnum::FuncExpr(node) => todo!("{:?}", node),
            NodeEnum::NamedArgExpr(node) => todo!("{:?}", node),
            NodeEnum::OpExpr(node) => todo!("{:?}", node),
            NodeEnum::DistinctExpr(node) => todo!("{:?}", node),
            NodeEnum::NullIfExpr(node) => todo!("{:?}", node),
            NodeEnum::ScalarArrayOpExpr(node) => todo!("{:?}", node),
            NodeEnum::BoolExpr(node) => todo!("{:?}", node),
            NodeEnum::SubLink(node) => todo!("{:?}", node),
            NodeEnum::SubPlan(node) => todo!("{:?}", node),
            NodeEnum::AlternativeSubPlan(node) => todo!("{:?}", node),
            NodeEnum::FieldSelect(node) => todo!("{:?}", node),
            NodeEnum::FieldStore(node) => todo!("{:?}", node),
            NodeEnum::RelabelType(node) => todo!("{:?}", node),
            NodeEnum::CoerceViaIo(node) => todo!("{:?}", node),
            NodeEnum::ArrayCoerceExpr(node) => todo!("{:?}", node),
            NodeEnum::ConvertRowtypeExpr(node) => todo!("{:?}", node),
            NodeEnum::CollateExpr(node) => todo!("{:?}", node),
            NodeEnum::CaseExpr(node) => todo!("{:?}", node),
            NodeEnum::CaseWhen(node) => todo!("{:?}", node),
            NodeEnum::CaseTestExpr(node) => todo!("{:?}", node),
            NodeEnum::ArrayExpr(node) => todo!("{:?}", node),
            NodeEnum::RowExpr(node) => todo!("{:?}", node),
            NodeEnum::RowCompareExpr(node) => todo!("{:?}", node),
            NodeEnum::CoalesceExpr(node) => todo!("{:?}", node),
            NodeEnum::MinMaxExpr(node) => todo!("{:?}", node),
            NodeEnum::SqlvalueFunction(node) => todo!("{:?}", node),
            NodeEnum::XmlExpr(node) => todo!("{:?}", node),
            NodeEnum::NullTest(node) => todo!("{:?}", node),
            NodeEnum::BooleanTest(node) => todo!("{:?}", node),
            NodeEnum::CoerceToDomain(node) => todo!("{:?}", node),
            NodeEnum::CoerceToDomainValue(node) => todo!("{:?}", node),
            NodeEnum::SetToDefault(node) => todo!("{:?}", node),
            NodeEnum::CurrentOfExpr(node) => todo!("{:?}", node),
            NodeEnum::NextValueExpr(node) => todo!("{:?}", node),
            NodeEnum::InferenceElem(node) => todo!("{:?}", node),
            NodeEnum::TargetEntry(node) => todo!("{:?}", node),
            NodeEnum::RangeTblRef(node) => todo!("{:?}", node),
            NodeEnum::JoinExpr(node) => todo!("{:?}", node),
            NodeEnum::FromExpr(node) => todo!("{:?}", node),
            NodeEnum::OnConflictExpr(node) => todo!("{:?}", node),
            NodeEnum::IntoClause(node) => todo!("{:?}", node),
            NodeEnum::MergeAction(node) => todo!("{:?}", node),
            NodeEnum::RawStmt(node) => todo!("{:?}", node),
            NodeEnum::Query(node) => todo!("{:?}", node),
            NodeEnum::InsertStmt(node) => todo!("{:?}", node),
            NodeEnum::DeleteStmt(node) => todo!("{:?}", node),
            NodeEnum::UpdateStmt(node) => todo!("{:?}", node),
            NodeEnum::MergeStmt(node) => todo!("{:?}", node),
            NodeEnum::SelectStmt(node) => node.print(p),
            NodeEnum::ReturnStmt(node) => todo!("{:?}", node),
            NodeEnum::PlassignStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterTableStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterTableCmd(node) => todo!("{:?}", node),
            NodeEnum::AlterDomainStmt(node) => todo!("{:?}", node),
            NodeEnum::SetOperationStmt(node) => todo!("{:?}", node),
            NodeEnum::GrantStmt(node) => todo!("{:?}", node),
            NodeEnum::GrantRoleStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterDefaultPrivilegesStmt(node) => todo!("{:?}", node),
            NodeEnum::ClosePortalStmt(node) => todo!("{:?}", node),
            NodeEnum::ClusterStmt(node) => todo!("{:?}", node),
            NodeEnum::CopyStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateStmt(node) => node.print(p),
            NodeEnum::DefineStmt(node) => node.print(p),
            NodeEnum::DropStmt(node) => todo!("{:?}", node),
            NodeEnum::TruncateStmt(node) => todo!("{:?}", node),
            NodeEnum::CommentStmt(node) => todo!("{:?}", node),
            NodeEnum::FetchStmt(node) => todo!("{:?}", node),
            NodeEnum::IndexStmt(node) => node.print(p),
            NodeEnum::CreateFunctionStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterFunctionStmt(node) => todo!("{:?}", node),
            NodeEnum::DoStmt(node) => todo!("{:?}", node),
            NodeEnum::RenameStmt(node) => todo!("{:?}", node),
            NodeEnum::RuleStmt(node) => todo!("{:?}", node),
            NodeEnum::NotifyStmt(node) => todo!("{:?}", node),
            NodeEnum::ListenStmt(node) => todo!("{:?}", node),
            NodeEnum::UnlistenStmt(node) => todo!("{:?}", node),
            NodeEnum::TransactionStmt(node) => todo!("{:?}", node),
            NodeEnum::ViewStmt(node) => node.print(p),
            NodeEnum::LoadStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateDomainStmt(node) => node.print(p),
            NodeEnum::CreatedbStmt(node) => todo!("{:?}", node),
            NodeEnum::DropdbStmt(node) => todo!("{:?}", node),
            NodeEnum::VacuumStmt(node) => todo!("{:?}", node),
            NodeEnum::ExplainStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateTableAsStmt(node) => node.print(p),
            NodeEnum::CreateSeqStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterSeqStmt(node) => todo!("{:?}", node),
            NodeEnum::VariableSetStmt(node) => todo!("{:?}", node),
            NodeEnum::VariableShowStmt(node) => todo!("{:?}", node),
            NodeEnum::DiscardStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateTrigStmt(node) => node.print(p),
            NodeEnum::CreatePlangStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateRoleStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterRoleStmt(node) => todo!("{:?}", node),
            NodeEnum::DropRoleStmt(node) => todo!("{:?}", node),
            NodeEnum::LockStmt(node) => todo!("{:?}", node),
            NodeEnum::ConstraintsSetStmt(node) => todo!("{:?}", node),
            NodeEnum::ReindexStmt(node) => todo!("{:?}", node),
            NodeEnum::CheckPointStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateSchemaStmt(node) => node.print(p),
            NodeEnum::AlterDatabaseStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterDatabaseRefreshCollStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterDatabaseSetStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterRoleSetStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateConversionStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateCastStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateOpClassStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateOpFamilyStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterOpFamilyStmt(node) => todo!("{:?}", node),
            NodeEnum::PrepareStmt(node) => todo!("{:?}", node),
            NodeEnum::ExecuteStmt(node) => node.print(p),
            NodeEnum::DeallocateStmt(node) => todo!("{:?}", node),
            NodeEnum::DeclareCursorStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateTableSpaceStmt(node) => todo!("{:?}", node),
            NodeEnum::DropTableSpaceStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterObjectDependsStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterObjectSchemaStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterOwnerStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterOperatorStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterTypeStmt(node) => todo!("{:?}", node),
            NodeEnum::DropOwnedStmt(node) => todo!("{:?}", node),
            NodeEnum::ReassignOwnedStmt(node) => todo!("{:?}", node),
            NodeEnum::CompositeTypeStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateEnumStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateRangeStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterEnumStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterTsdictionaryStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterTsconfigurationStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateFdwStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterFdwStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateForeignServerStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterForeignServerStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateUserMappingStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterUserMappingStmt(node) => todo!("{:?}", node),
            NodeEnum::DropUserMappingStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterTableSpaceOptionsStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterTableMoveAllStmt(node) => todo!("{:?}", node),
            NodeEnum::SecLabelStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateForeignTableStmt(node) => todo!("{:?}", node),
            NodeEnum::ImportForeignSchemaStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateExtensionStmt(node) => node.print(p),
            NodeEnum::AlterExtensionStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterExtensionContentsStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateEventTrigStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterEventTrigStmt(node) => todo!("{:?}", node),
            NodeEnum::RefreshMatViewStmt(node) => todo!("{:?}", node),
            NodeEnum::ReplicaIdentityStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterSystemStmt(node) => todo!("{:?}", node),
            NodeEnum::CreatePolicyStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterPolicyStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateTransformStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateAmStmt(node) => todo!("{:?}", node),
            NodeEnum::CreatePublicationStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterPublicationStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateSubscriptionStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterSubscriptionStmt(node) => todo!("{:?}", node),
            NodeEnum::DropSubscriptionStmt(node) => todo!("{:?}", node),
            NodeEnum::CreateStatsStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterCollationStmt(node) => todo!("{:?}", node),
            NodeEnum::CallStmt(node) => todo!("{:?}", node),
            NodeEnum::AlterStatsStmt(node) => todo!("{:?}", node),
            NodeEnum::AExpr(node) => node.print(p),
            NodeEnum::ColumnRef(node) => node.print(p),
            NodeEnum::ParamRef(node) => todo!("{:?}", node),
            NodeEnum::FuncCall(node) => todo!("{:?}", node),
            NodeEnum::AStar(node) => todo!("{:?}", node),
            NodeEnum::AIndices(node) => todo!("{:?}", node),
            NodeEnum::AIndirection(node) => todo!("{:?}", node),
            NodeEnum::AArrayExpr(node) => todo!("{:?}", node),
            NodeEnum::ResTarget(node) => node.print(p),
            NodeEnum::MultiAssignRef(node) => todo!("{:?}", node),
            NodeEnum::TypeCast(node) => todo!("{:?}", node),
            NodeEnum::CollateClause(node) => todo!("{:?}", node),
            NodeEnum::SortBy(node) => todo!("{:?}", node),
            NodeEnum::WindowDef(node) => todo!("{:?}", node),
            NodeEnum::RangeSubselect(node) => todo!("{:?}", node),
            NodeEnum::RangeFunction(node) => todo!("{:?}", node),
            NodeEnum::RangeTableSample(node) => todo!("{:?}", node),
            NodeEnum::RangeTableFunc(node) => todo!("{:?}", node),
            NodeEnum::RangeTableFuncCol(node) => todo!("{:?}", node),
            NodeEnum::TypeName(node) => todo!("{:?}", node),
            NodeEnum::ColumnDef(node) => node.print(p),
            NodeEnum::IndexElem(node) => node.print(p),
            NodeEnum::StatsElem(node) => todo!("{:?}", node),
            NodeEnum::Constraint(node) => node.print(p),
            NodeEnum::DefElem(node) => node.print(p),
            NodeEnum::RangeTblEntry(node) => todo!("{:?}", node),
            NodeEnum::RangeTblFunction(node) => todo!("{:?}", node),
            NodeEnum::TableSampleClause(node) => todo!("{:?}", node),
            NodeEnum::WithCheckOption(node) => todo!("{:?}", node),
            NodeEnum::SortGroupClause(node) => todo!("{:?}", node),
            NodeEnum::GroupingSet(node) => todo!("{:?}", node),
            NodeEnum::WindowClause(node) => todo!("{:?}", node),
            NodeEnum::ObjectWithArgs(node) => todo!("{:?}", node),
            NodeEnum::AccessPriv(node) => todo!("{:?}", node),
            NodeEnum::CreateOpClassItem(node) => todo!("{:?}", node),
            NodeEnum::TableLikeClause(node) => todo!("{:?}", node),
            NodeEnum::FunctionParameter(node) => todo!("{:?}", node),
            NodeEnum::LockingClause(node) => todo!("{:?}", node),
            NodeEnum::RowMarkClause(node) => todo!("{:?}", node),
            NodeEnum::XmlSerialize(node) => todo!("{:?}", node),
            NodeEnum::WithClause(node) => todo!("{:?}", node),
            NodeEnum::InferClause(node) => todo!("{:?}", node),
            NodeEnum::OnConflictClause(node) => todo!("{:?}", node),
            NodeEnum::CtesearchClause(node) => todo!("{:?}", node),
            NodeEnum::CtecycleClause(node) => todo!("{:?}", node),
            NodeEnum::CommonTableExpr(node) => todo!("{:?}", node),
            NodeEnum::MergeWhenClause(node) => todo!("{:?}", node),
            NodeEnum::RoleSpec(node) => todo!("{:?}", node),
            NodeEnum::TriggerTransition(node) => todo!("{:?}", node),
            NodeEnum::PartitionElem(node) => todo!("{:?}", node),
            NodeEnum::PartitionSpec(node) => todo!("{:?}", node),
            NodeEnum::PartitionBoundSpec(node) => todo!("{:?}", node),
            NodeEnum::PartitionRangeDatum(node) => todo!("{:?}", node),
            NodeEnum::PartitionCmd(node) => todo!("{:?}", node),
            NodeEnum::VacuumRelation(node) => todo!("{:?}", node),
            NodeEnum::PublicationObjSpec(node) => todo!("{:?}", node),
            NodeEnum::PublicationTable(node) => todo!("{:?}", node),
            NodeEnum::InlineCodeBlock(node) => todo!("{:?}", node),
            NodeEnum::CallContext(node) => todo!("{:?}", node),
            NodeEnum::Integer(node) => node.print(p),
            NodeEnum::Float(node) => todo!("{:?}", node),
            NodeEnum::Boolean(node) => todo!("{:?}", node),
            NodeEnum::String(node) => todo!("{:?}", node),
            NodeEnum::BitString(node) => todo!("{:?}", node),
            NodeEnum::List(node) => todo!("{:?}", node),
            NodeEnum::IntList(node) => todo!("{:?}", node),
            NodeEnum::OidList(node) => todo!("{:?}", node),
            NodeEnum::AConst(node) => node.print(p),
        }
    }
}

impl fmt::Print for [Node] {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        utils::print_expr_list(p, self)
    }
}
