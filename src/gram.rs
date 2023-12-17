// Adapted from https://github.com/postgres/postgres/blob/REL_15_STABLE/src/backend/parser/gram.y.

use crate::fmt::Context;
use crate::fmt::Printer;
use crate::interval_fields::IntervalFields;
use crate::interval_fields::INTERVAL_FULL_PRECISION;
use crate::name::Name;
use crate::partition::PartitionStrategy;
use crate::rel_persistence::RelPersistence;
use crate::INDENT;
use pg_query::protobuf;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::AConst;
use pg_query::protobuf::AStar;
use pg_query::protobuf::AccessPriv;
use pg_query::protobuf::BoolExprType;
use pg_query::protobuf::CollateClause;
use pg_query::protobuf::ColumnDef;
use pg_query::protobuf::ColumnRef;
use pg_query::protobuf::ConstrType;
use pg_query::protobuf::Constraint;
use pg_query::protobuf::DefElem;
use pg_query::protobuf::DropBehavior;
use pg_query::protobuf::FunctionParameter;
use pg_query::protobuf::FunctionParameterMode;
use pg_query::protobuf::GrantTargetType;
use pg_query::protobuf::IndexElem;
use pg_query::protobuf::Integer;
use pg_query::protobuf::IntoClause;
use pg_query::protobuf::List;
use pg_query::protobuf::ObjectType;
use pg_query::protobuf::OnCommitAction;
use pg_query::protobuf::PartitionBoundSpec;
use pg_query::protobuf::RangeVar;
use pg_query::protobuf::ResTarget;
use pg_query::protobuf::RoleSpec;
use pg_query::protobuf::RoleSpecType;
use pg_query::protobuf::SortByDir;
use pg_query::protobuf::SortByNulls;
use pg_query::protobuf::TypeName;
use pg_query::protobuf::WithClause;
use pg_query::Node;
use pg_query::NodeEnum;

pub const TRIGGER_TYPE_BEFORE: usize = 1 << 1;
pub const TRIGGER_TYPE_INSERT: usize = 1 << 2;
pub const TRIGGER_TYPE_DELETE: usize = 1 << 3;
pub const TRIGGER_TYPE_UPDATE: usize = 1 << 4;
pub const TRIGGER_TYPE_TRUNCATE: usize = 1 << 5;
pub const TRIGGER_TYPE_INSTEAD: usize = 1 << 6;
pub const TRIGGER_TYPE_AFTER: usize = 0;

const NAMEDATALEN: usize = 64;
const ESCAPE_STRING_SYNTAX: char = 'E';

/// Returns the given expression if it matches any of the given patterns.
#[macro_export]
macro_rules! cast {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $expression {
            $pattern $(if $guard)? => Some($expression),
            _ => None
        }
    };
}

#[macro_export]
macro_rules! cast_node {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $expression {
            pg_query::Node { node: Some($pattern) } $(if $guard)? => Some($expression),
            _ => None
        }
    };
}

// See https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c#L53.
pub fn string_literal(p: &mut Printer, val: &str) {
    if val.contains('\\') {
        p.word(ESCAPE_STRING_SYNTAX.to_string());
    }

    p.word('\''.to_string());

    for c in val.chars() {
        if c == '\'' || c == '\\' {
            p.word(c.to_string());
        }

        p.word(c.to_string());
    }

    p.word('\''.to_string());
}

pub fn is_op(val: Option<String>) -> bool {
    val.unwrap().chars().all(|cp| {
        cp == '~'
            || cp == '!'
            || cp == '@'
            || cp == '#'
            || cp == '^'
            || cp == '&'
            || cp == '|'
            || cp == '`'
            || cp == '?'
            || cp == '+'
            || cp == '-'
            || cp == '*'
            || cp == '/'
            || cp == '%'
            || cp == '<'
            || cp == '>'
            || cp == '='
    })
}

pub fn str_val(node: &Node) -> Option<String> {
    match node.node.as_ref().unwrap() {
        NodeEnum::String(val) => Some(val.sval.clone()),
        _ => None,
    }
}

pub fn int_val(node: &Node) -> Option<i32> {
    match node.node.as_ref().unwrap() {
        NodeEnum::Integer(val) => Some(val.ival),
        _ => None,
    }
}

pub fn a_const_int_val(node: &Node) -> Option<i32> {
    match node.node.as_ref().unwrap() {
        NodeEnum::AConst(AConst {
            val: Some(Val::Ival(val)),
            ..
        }) => Some(val.ival),
        _ => None,
    }
}

pub fn make_range_var_into_any_name(n: &RangeVar) -> [Node; 1] {
    [Node {
        node: Some(NodeEnum::String(protobuf::String {
            sval: n.relname.clone(),
        })),
    }]
}

impl Printer {
    pub fn a_const(&mut self, n: &AConst) {
        self.opt_val(n.val.as_ref(), &Context::Constant);
    }

    pub fn a_star(&mut self, _n: &AStar) {
        self.word("*");
    }

    pub fn any_name(&mut self, list: &[Node]) {
        for (i, part) in list.iter().enumerate() {
            if i > 0 {
                self.word(".");
            }

            self.ident(str_val(part).unwrap());
        }
    }

    pub fn expr_list(&mut self, list: &[Node]) {
        for (i, expr) in list.iter().enumerate() {
            self.node(expr);
            self.trailing_comma(i >= list.len() - 1);
        }
    }

    pub fn column_list(&mut self, list: &[Node]) {
        for (i, col) in list.iter().enumerate() {
            self.ident(str_val(col).unwrap());

            if i < list.len() - 1 {
                self.word(", ");
            }
        }
    }

    pub fn print_list(&mut self, list: &[Node]) {
        self.expr_list(list);
    }

    pub fn opt_with(&mut self, list: &[Node]) {
        if !list.is_empty() {
            self.word(" with ");
            self.word("(");

            for (i, option) in list.iter().enumerate() {
                self.node(option);
                self.trailing_comma(i >= list.len() - 1);
            }

            self.word(")");
            self.nbsp();
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_clause(&mut self, list: &[Node]) {
        if !list.is_empty() {
            self.word("from ");

            for (i, item) in list.iter().enumerate() {
                self.node(item);
                self.trailing_comma(i >= list.len() - 1);
            }
            self.word(" ");
        }
    }

    pub fn where_clause(&mut self, n: Option<&Node>) {
        if let Some(node) = n {
            self.word("where ");
            self.node(node);
            self.word(" ");
        }
    }

    pub fn opt_collate(&mut self, list: &[Node]) {
        if !list.is_empty() {
            self.word("collate ");
            self.any_name(list);
            self.nbsp();
        }
    }

    pub fn reloptions(&mut self, _list: &[Node]) {
        todo!();
    }

    pub fn func_name(&mut self, list: &[Node]) {
        for (i, part) in list.iter().enumerate() {
            if i > 0 {
                self.word(".");
            }

            self.ident(str_val(part).unwrap());
        }
    }

    pub fn func_args_with_defaults(&mut self, list: &[Node]) {
        self.word("(");
        if !list.is_empty() {
            self.cbox(INDENT);
            self.hardbreak_if_nonempty();
            for (i, arg) in list.iter().enumerate() {
                self.node(arg);
                self.trailing_comma(i >= list.len() - 1);
            }
            self.space();
            self.offset(-INDENT);
            self.end();
        }
        self.word(")");
        self.space();
    }

    pub fn func_return(&mut self, n: &TypeName) {
        self.type_name(n);
        self.nbsp();
    }

    pub fn opt_createfunc_opt_list(&mut self, list: &[Node]) {
        if !list.is_empty() {
            for option in list.iter().skip(1) {
                if let NodeEnum::DefElem(node) = option.node.as_ref().unwrap() {
                    if let Some(arg) = &node.arg {
                        if node.defname == "volatility" && str_val(arg).unwrap() == "stable" {
                        } else {
                            self.word(node.defname.clone());
                            self.nbsp();
                            self.word(str_val(arg).unwrap());
                        }
                    }
                }
            }
            if let Some(option) = list.first() {
                if let NodeEnum::DefElem(node) = option.node.as_ref().unwrap() {
                    if let Some(arg) = &node.arg {
                        if node.defname == "as" {
                            self.hardbreak_if_nonempty();
                            self.word(node.defname.clone());
                            self.cbox(INDENT);
                            self.nbsp();
                            self.word("$$");
                            self.hardbreak_if_nonempty();
                            if let NodeEnum::List(List { items }) = arg.node.as_ref().unwrap() {
                                self.word(
                                    str_val(items.first().unwrap()).unwrap().trim().to_owned(),
                                );
                            }
                            self.hardbreak();
                            self.offset(-INDENT);
                            self.word("$$");
                        }
                    }
                    self.nbsp();
                }
            }
        }
    }

    pub fn opt_routine_body(&mut self, n: Option<&Node>) {
        if let Some(node) = n {
            self.word("as ");
            self.node(node);
            self.nbsp();
        }
    }

    pub fn arg_class(&mut self, n: &FunctionParameterMode) {
        self.function_parameter_mode(n);
    }

    pub fn param_name(&mut self, val: &str) {
        self.ident(val.to_string());
        self.word(" ");
    }

    pub fn func_type(&mut self, n: &TypeName) {
        self.type_name(n);
    }

    pub fn opt_drop_behavior(&mut self, n: &DropBehavior) {
        match n {
            DropBehavior::DropRestrict => {}
            DropBehavior::DropCascade => self.word("cascade "),
            _ => {}
        }
    }

    pub fn name_list(&mut self, list: &[Node]) {
        for (i, name) in list.iter().enumerate() {
            self.ident(str_val(name).unwrap());
            self.trailing_comma(i >= list.len() - 1);
        }
    }

    pub fn qual_op(&mut self, list: &[Node]) {
        if list.len() == 1 && is_op(str_val(list.first().unwrap())) {
            self.word(str_val(list.first().unwrap()).unwrap());
        } else {
            self.word("operator(");
            self.any_operator(list);
            self.word(")");
        }
    }

    pub fn any_operator(&mut self, list: &[Node]) {
        match list.len() {
            2 => {
                self.ident(str_val(list.first().unwrap()).unwrap());
                self.word(".");
                self.word(str_val(list.last().unwrap()).unwrap());
            }
            1 => {
                self.ident(str_val(list.last().unwrap()).unwrap());
            }
            _ => unreachable!(),
        }
    }

    pub fn create_generic_options(&mut self, list: &[Node]) {
        if !list.is_empty() {
            self.word("options ");
            self.word("(");
            self.generic_option_list(list);
            self.word(") ");
        }
    }

    pub fn generic_option_list(&mut self, list: &[Node]) {
        self.qualified_name_list(list);
    }

    pub fn opt_temp(&mut self, relpersistence: String) {
        self.rel_persistence(&RelPersistence::from(relpersistence))
    }

    pub fn non_reserved_word_or_scont(&mut self, val: String) {
        match val.len() {
            0 => self.word("''".to_string()),
            x if x > NAMEDATALEN => string_literal(self, &val),
            _ => self.ident(val),
        }
    }

    pub fn opt_inherit(&mut self, list: &[Node]) {
        if !list.is_empty() {
            self.word("inherits ");
            self.word("(");
            self.qualified_name_list(list);
            self.word(") ");
        }
    }

    pub fn signed_iconst(&mut self, n: &Node) {
        self.word(format!("{}", int_val(n).unwrap()));
    }

    pub fn qualified_name_list(&mut self, list: &[Node]) {
        for (i, name) in list.iter().enumerate() {
            self.qualified_name(name);
            if i < list.len() - 1 {
                self.word(",");
                self.nbsp();
            }
        }
    }

    pub fn qualified_name(&mut self, n: &Node) {
        self.node(n);
    }

    pub fn name(&mut self, name: String) {
        self.ident(name);
    }

    pub fn opt_or_replace(&mut self, replace: bool) {
        if replace {
            self.word("or replace ");
        }
    }

    pub fn opt_table_func_element_list(&mut self, list: &[Node]) {
        if !list.is_empty() {
            self.table_func_element_list(list)
        }
    }

    pub fn table_func_element_list(&mut self, list: &[Node]) {
        for (i, n) in list.iter().enumerate() {
            let n = cast_node!(n, NodeEnum::ColumnDef(n)).unwrap();
            self.table_func_element(n);
            self.trailing_comma(i >= list.len() - 1)
        }
    }

    pub fn table_func_element(&mut self, n: &ColumnDef) {
        self.col_id(&n.colname);
        self.nbsp();
        if let Some(type_name) = &n.type_name {
            self.type_name(type_name);
        }
        self.opt_collate_clause(n.coll_clause.as_deref());
    }

    pub fn col_id(&mut self, s: &str) {
        self.ident(s.to_owned())
    }

    pub fn opt_as(&mut self) {
        self.word(" as ")
    }

    pub fn col_qual_list(&mut self, list: &[Node], col: Option<&CollateClause>) {
        for node in list.iter() {
            if let Some(NodeEnum::Constraint(node)) = &node.node {
                self.col_constraint(node, col)
            }
        }
    }

    fn col_constraint(&mut self, n: &Constraint, _col: Option<&CollateClause>) {
        self.col_constraint_elem(n);
    }

    fn col_constraint_elem(&mut self, n: &Constraint) {
        match n.contype() {
            ConstrType::ConstrCheck => {
                self.word("check ");
                self.word("(");
                let expr_list = &n
                    .raw_expr
                    .as_ref()
                    .and_then(|node| node.node.as_ref())
                    .and_then(|node| cast!(node, NodeEnum::BoolExpr(node)))
                    .map(|expr| expr.args.clone())
                    .into_iter()
                    .flatten()
                    .map(|node| node.node)
                    .filter_map(|node| cast!(node, Some(NodeEnum::AExpr(node))))
                    .collect::<Vec<_>>();
                for (i, expr) in expr_list.iter().enumerate() {
                    self.a_expr(expr, &Context::AExpr);

                    match n.raw_expr.as_ref().unwrap().node.as_ref().unwrap() {
                        NodeEnum::BoolExpr(node) => match &node.boolop() {
                            BoolExprType::OrExpr => {
                                if i < expr_list.len() - 1 {
                                    self.word(" or ")
                                }
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                self.word(")");
                self.opt_no_inherit(n.is_no_inherit);
            }
            _ => todo!(),
        }
    }

    pub fn opt_collate_clause(&mut self, n: Option<&CollateClause>) {
        if let Some(n) = n {
            self.collate_clause(n)
        }
    }

    pub fn collate_clause(&mut self, n: &CollateClause) {
        if let Some(arg) = &n.arg {
            let need_parens = matches!(arg.node.as_ref().unwrap(), NodeEnum::AExpr(_));

            self.optional_word("(", need_parens);
            self.node(arg);
            self.optional_word(")", need_parens);
            self.nbsp();
        }

        self.word("collate ");
        self.any_name(&n.collname);
    }

    pub fn on_commit_option(&mut self, n: &OnCommitAction) {
        match n {
            OnCommitAction::Undefined => {}
            OnCommitAction::OncommitNoop => {}
            OnCommitAction::OncommitPreserveRows => self.word(" on commit preserve rows"),
            OnCommitAction::OncommitDeleteRows => self.word(" on commit delete rows"),
            OnCommitAction::OncommitDrop => self.word(" on commit drop"),
        }
    }

    pub fn column_def(&mut self, n: &ColumnDef) {
        if !n.colname.is_empty() {
            self.ident(n.colname.clone());
        }

        if let Some(type_name) = &n.type_name {
            self.nbsp();
            self.type_name(type_name);
        }

        if let Some(raw_default) = &n.raw_default {
            self.nbsp();
            self.word("using ");
            self.node(raw_default);
        }

        if !n.fdwoptions.is_empty() {
            self.nbsp();
            self.create_generic_options(&n.fdwoptions);
        }

        for constraint in n.constraints.iter() {
            self.nbsp();
            self.node(constraint);
        }

        if let Some(coll_clause) = &n.coll_clause {
            self.collate_clause(coll_clause);
        }
    }

    pub fn column_ref(&mut self, n: &ColumnRef) {
        if let NodeEnum::AStar(node) = n.fields.first().unwrap().node.as_ref().unwrap() {
            self.a_star(node);
        } else if let NodeEnum::String(node) = n.fields.first().unwrap().node.as_ref().unwrap() {
            self.col_label(&node.sval);
        }

        self.opt_indirection(&n.fields, 1);
    }

    pub fn col_label(&mut self, node: &str) {
        self.ident(node.to_owned());
    }

    pub fn opt_indirection(&mut self, _list: &[Node], _offset: usize) {
        // for (i, item) in list.iter().enumerate().skip(offset) {}
    }

    pub fn constraint(&mut self, n: &Constraint) {
        if !n.conname.is_empty() {
            self.word("constraint ");
            self.ident(n.conname.clone());
            self.nbsp();
        }

        match n.contype() {
            ConstrType::ConstrDefault => {
                self.word("default ");
                if let Some(raw_expr) = &n.raw_expr {
                    self.node(raw_expr);
                }
            }
            ConstrType::ConstrPrimary => {
                self.word("primary key");
            }
            ConstrType::ConstrUnique => {
                self.word("unique");
            }
            ConstrType::ConstrCheck => {
                self.word("check (");
                if let Some(raw_expr) = &n.raw_expr {
                    self.node(raw_expr);
                }
                self.word(")");
            }
            ConstrType::ConstrNotnull => {
                self.word("not null");
            }
            _ => todo!(),
        }

        if !n.keys.is_empty() {
            self.nbsp();
            self.word("(");
            self.column_list(&n.keys);
            self.word(")");
        }

        match n.contype() {
            ConstrType::ConstrPrimary | ConstrType::ConstrUnique | ConstrType::ConstrExclusion => {
                self.opt_with(&n.options);
            }
            _ => {}
        }

        if !n.indexspace.is_empty() {
            self.word("using index tablespace ");
            self.ident(n.indexspace.clone());
        }
    }

    pub fn opt_no_inherit(&mut self, no_inherit: bool) {
        if no_inherit {
            self.word("no inherit ");
        }
    }

    pub fn def_elem(&mut self, n: &DefElem) {
        if !n.defnamespace.is_empty() {
            self.ident(n.defnamespace.clone());
            self.word(".");
        }

        self.ident(n.defname.clone());

        if let Some(arg) = &n.arg {
            self.word(" = ");
            self.node(arg);
        }
    }

    pub fn function_parameter(&mut self, n: &FunctionParameter) {
        self.arg_class(&n.mode());
        self.param_name(&n.name);
        self.func_type(n.arg_type.as_ref().unwrap());
    }

    pub fn function_parameter_mode(&mut self, n: &FunctionParameterMode) {
        match n {
            FunctionParameterMode::FuncParamIn => self.word("in "),
            FunctionParameterMode::FuncParamOut => self.word("out "),
            FunctionParameterMode::FuncParamInout => self.word("inout "),
            FunctionParameterMode::FuncParamVariadic => self.word("variadic "),
            _ => {}
        }
    }

    pub fn index_elem(&mut self, n: &IndexElem) {
        if !n.name.is_empty() {
            self.ident(n.name.clone());
        } else if let Some(expr) = &n.expr {
            self.node(expr);
        } else {
            unreachable!();
        }

        self.opt_collate(&n.collation);

        if !n.opclass.is_empty() {
            self.any_name(&n.opclass);

            if !n.opclassopts.is_empty() {
                self.reloptions(&n.opclassopts);
            }

            self.nbsp();
        }

        match n.ordering() {
            SortByDir::SortbyAsc => self.word("asc "),
            SortByDir::SortbyDesc => self.word("desc "),
            _ => {}
        }

        match n.nulls_ordering() {
            SortByNulls::SortbyNullsFirst => self.word("nulls first "),
            SortByNulls::SortbyNullsLast => self.word("nulls last "),
            _ => {}
        }
    }

    pub fn integer(&mut self, n: &Integer) {
        self.opt_val(Some(&Val::Ival(n.clone())), &Context::None);
    }

    pub fn string(&mut self, n: &protobuf::String) {
        self.word("'");
        self.ident(n.sval.clone());
        self.word("'");
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_clause(&mut self, n: &IntoClause) {
        if let Some(rel) = &n.rel {
            self.range_var(rel);
        }

        if !n.col_names.is_empty() {
            self.word(" (");
            self.column_list(&n.col_names);
            self.word(")");
        }

        if !n.access_method.is_empty() {
            self.word("using ");
            self.ident(n.access_method.clone());
            self.word(" ");
        }

        self.opt_with(&n.options);

        self.on_commit_option(&n.on_commit());

        if !n.table_space_name.is_empty() {
            self.word("tablespace ");
            self.ident(n.table_space_name.clone());
            self.word(" ");
        }
    }

    pub fn list(&mut self, n: &List) {
        for item in &n.items {
            self.node(item);
        }
    }

    pub fn object_type(&mut self, n: &ObjectType) {
        match n {
            ObjectType::ObjectAggregate => self.word("aggregate "),
            ObjectType::ObjectOperator => self.word("operator "),
            ObjectType::ObjectType => self.word("type "),
            ObjectType::ObjectTsparser => self.word("text search parser "),
            ObjectType::ObjectTsdictionary => self.word("text seach dictionary "),
            ObjectType::ObjectTstemplate => self.word("text search template "),
            ObjectType::ObjectTsconfiguration => self.word("text search configuration "),
            ObjectType::ObjectCollation => self.word("collation "),
            ObjectType::ObjectTable => self.word("table "),
            ObjectType::ObjectMatview => self.word("materialized view "),
            _ => todo!(),
        }
    }

    pub fn access_priv(&mut self, n: &AccessPriv) {
        if !n.priv_name.is_empty() {
            match n.priv_name.as_ref() {
                "select" => self.word("select"),
                "references" => self.word("references"),
                "create" => self.word("create"),
                _ => self.ident(n.priv_name.clone()),
            }
        } else {
            self.word("all")
        }

        self.nbsp();

        if !n.cols.is_empty() {
            self.word("(");
            self.column_list(&n.cols);
            self.word(")");
        }
    }

    pub fn privilege_target(
        &mut self,
        targtype: &GrantTargetType,
        objtype: &ObjectType,
        objs: &[Node],
    ) {
        match targtype {
            GrantTargetType::AclTargetObject => match objtype {
                ObjectType::ObjectTable => self.print_list(objs),
                ObjectType::ObjectSequence => todo!(),
                ObjectType::ObjectFdw => todo!(),
                ObjectType::ObjectForeignServer => todo!(),
                ObjectType::ObjectFunction => todo!(),
                ObjectType::ObjectProcedure => todo!(),
                ObjectType::ObjectRoutine => todo!(),
                ObjectType::ObjectDatabase => todo!(),
                ObjectType::ObjectDomain => todo!(),
                ObjectType::ObjectLanguage => todo!(),
                ObjectType::ObjectLargeobject => todo!(),
                ObjectType::ObjectSchema => {
                    self.word("schema ");
                    self.name_list(objs);
                }
                ObjectType::ObjectTablespace => todo!(),
                ObjectType::ObjectType => todo!(),
                _ => {}
            },
            GrantTargetType::AclTargetAllInSchema => match objtype {
                ObjectType::ObjectTable => todo!(),
                ObjectType::ObjectSequence => todo!(),
                ObjectType::ObjectFunction => todo!(),
                ObjectType::ObjectProcedure => todo!(),
                ObjectType::ObjectRoutine => todo!(),
                _ => {}
            },
            GrantTargetType::AclTargetDefaults => match objtype {
                ObjectType::ObjectTable => todo!(),
                ObjectType::ObjectFunction => todo!(),
                ObjectType::ObjectSequence => todo!(),
                ObjectType::ObjectType => todo!(),
                ObjectType::ObjectSchema => todo!(),
                _ => {}
            },
            _ => {}
        }
    }

    pub fn range_var(&mut self, n: &RangeVar) {
        self.ident(n.relname.clone());
    }

    pub fn res_target(&mut self, n: &ResTarget) {
        if n.val.is_none() {
        } else if let NodeEnum::ColumnRef(node) = n.val.as_ref().unwrap().node.as_ref().unwrap() {
            self.column_ref(node);
        } else {
            self.node(n.val.as_deref().unwrap());
        }

        if !n.name.is_empty() {
            self.word(" as ");
            self.ident(n.name.clone());
        }
    }

    pub fn role_spec(&mut self, n: &RoleSpec) {
        match n.roletype() {
            RoleSpecType::RolespecCstring => self.ident(n.rolename.clone()),
            RoleSpecType::RolespecCurrentRole => self.word("current_role"),
            RoleSpecType::RolespecCurrentUser => self.word("current_user"),
            RoleSpecType::RolespecSessionUser => self.word("session_user"),
            RoleSpecType::RolespecPublic => self.word("public"),
            _ => {}
        }
    }

    pub fn with_clause(&mut self, n: &WithClause) {
        self.word("with ");

        if n.recursive {
            self.word("recursive ");
        }

        todo!();
    }

    pub fn partition_bound_spec(&mut self, n: &PartitionBoundSpec) {
        if n.is_default {
            return self.word("default");
        }

        self.word(" for values ");

        match n.strategy.clone().into() {
            PartitionStrategy::Hash => {
                self.word(format!(
                    "with (modulus {}, remainder {})",
                    n.modulus, n.remainder
                ));
            }
            PartitionStrategy::List => {
                self.word("in (");
                self.print_list(&n.listdatums);
                self.word(")");
            }
            PartitionStrategy::Range => {
                self.word("from (");
                self.print_list(&n.lowerdatums);
                self.word(") to (");
                self.print_list(&n.upperdatums);
                self.word(")");
            }
            _ => unimplemented!("unknown PartitionStrategy"),
        }
    }

    pub fn rel_persistence(&mut self, n: &RelPersistence) {
        match n {
            RelPersistence::Temp => self.word("temporary "),
            RelPersistence::Unlogged => self.word("unlogged "),
            RelPersistence::Permanent => {}
            RelPersistence::Undefined => unreachable!(),
        }
    }

    pub fn type_name(&mut self, n: &TypeName) {
        let mut skip_typmods = false;

        if n.setof {
            self.word("setof ");
        }

        if n.names.len() == 2 && str_val(n.names.first().unwrap()).unwrap() == "pg_catalog" {
            let name = str_val(n.names.last().unwrap()).unwrap();

            match name.clone().into() {
                Name::Bpchar => self.word("char"),
                Name::Varchar => self.word("varchar"),
                Name::Numeric => self.word("numeric"),
                Name::Bool => self.word("boolean"),
                Name::Int2 => self.word("smallint"),
                Name::Int4 => self.word("int"),
                Name::Int8 => self.word("bigint"),
                Name::Real => self.word("real"),
                Name::Float8 => self.word("double precision"),
                Name::Time => self.word("time"),
                Name::Timetz => {
                    skip_typmods = true;
                    self.word("time ");

                    if !n.typmods.is_empty() {
                        self.word("(");
                        for (i, typmod) in n.typmods.iter().enumerate() {
                            self.signed_iconst(typmod);
                            self.trailing_comma(i >= n.typmods.len() - 1);
                        }
                        self.word(") ");
                    }

                    self.word("with time zone")
                }
                Name::Timestamp => self.word("timestamp"),
                Name::Timestamptz => {
                    skip_typmods = true;
                    self.word("timestamp ");

                    if !n.typmods.is_empty() {
                        self.word("(");
                        for (i, typmod) in n.typmods.iter().enumerate() {
                            self.signed_iconst(typmod);
                            self.trailing_comma(i >= n.typmods.len() - 1);
                        }
                        self.word(") ");
                    }
                    self.word("with time zone")
                }
                Name::Interval => {
                    self.word("interval");

                    if !n.typmods.is_empty() {
                        skip_typmods = true;
                        self.interval_typmods(n);
                    }
                }
                Name::Undefined => {
                    self.word("pg_catalog.");
                    self.word(name)
                }
            };
        } else {
            self.any_name(&n.names);
        }

        if !n.typmods.is_empty() && !skip_typmods {
            self.word("(");
            for (i, typmod) in n.typmods.iter().enumerate() {
                self.node(typmod);
                self.trailing_comma(i >= n.typmods.len() - 1);
            }
            self.word(")");
        }
    }

    pub fn interval_typmods(&mut self, node: &TypeName) {
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

        self.interval_fields(&IntervalFields::from(interval_fields));

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
                self.word(format!(" ({})", precision));
            }
        }
    }

    pub fn val(&mut self, val: &Val, context: &Context) {
        match val {
            Val::Ival(val) => self.word(format!("{}", val.ival)),
            Val::Fval(val) => self.word(val.fval.clone()),
            Val::Boolval(val) => self.word(if val.boolval { "true" } else { "false" }),
            Val::Sval(val) => match context {
                Context::Identifier => self.ident(val.sval.clone()),
                Context::Constant => string_literal(self, &val.sval),
                _ => self.word(val.sval.clone()),
            },
            Val::Bsval(val) => match val.bsval.chars().next().unwrap() {
                'x' => {
                    self.word("x");
                    string_literal(self, &val.bsval[1..])
                }
                'b' => {
                    self.word("b");
                    string_literal(self, &val.bsval[1..])
                }
                _ => unreachable!(),
            },
        }
    }

    pub fn opt_val(&mut self, val: Option<&Val>, context: &Context) {
        match val {
            Some(val) => self.val(val, context),
            None => self.word("null"),
        }
    }

    pub fn interval_fields(&mut self, n: &IntervalFields) {
        match n {
            IntervalFields::Year => self.word(" year"),
            IntervalFields::Month => self.word(" month"),
            IntervalFields::Day => self.word(" day"),
            IntervalFields::Hour => self.word(" hour"),
            IntervalFields::Minute => self.word(" minute"),
            IntervalFields::Second => self.word(" second"),
            IntervalFields::YearToMonth => self.word(" year to month"),
            IntervalFields::DayToHour => self.word(" day to hour"),
            IntervalFields::DayToMinute => self.word(" day to minute"),
            IntervalFields::DayToSecond => self.word(" day to second"),
            IntervalFields::HourToMinute => self.word(" hour to minute"),
            IntervalFields::HourToSecond => self.word(" hour to second"),
            IntervalFields::MinuteToSecond => self.word(" minute to second"),
            IntervalFields::FullRange => {}
            IntervalFields::Undefined => unreachable!(),
        }
    }
}
