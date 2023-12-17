use crate::fmt::Printer;
use crate::gram::make_range_var_into_any_name;
use crate::gram::str_val;
use crate::gram::string_literal;
use crate::gram::TRIGGER_TYPE_AFTER;
use crate::gram::TRIGGER_TYPE_BEFORE;
use crate::gram::TRIGGER_TYPE_DELETE;
use crate::gram::TRIGGER_TYPE_INSERT;
use crate::gram::TRIGGER_TYPE_INSTEAD;
use crate::gram::TRIGGER_TYPE_TRUNCATE;
use crate::gram::TRIGGER_TYPE_UPDATE;
use crate::INDENT;
use pg_query::protobuf::CompositeTypeStmt;
use pg_query::protobuf::CreateDomainStmt;
use pg_query::protobuf::CreateExtensionStmt;
use pg_query::protobuf::CreateForeignTableStmt;
use pg_query::protobuf::CreateFunctionStmt;
use pg_query::protobuf::CreateSchemaStmt;
use pg_query::protobuf::CreateStmt;
use pg_query::protobuf::CreateTableAsStmt;
use pg_query::protobuf::CreateTrigStmt;
use pg_query::protobuf::DefineStmt;
use pg_query::protobuf::ExecuteStmt;
use pg_query::protobuf::GrantStmt;
use pg_query::protobuf::IndexStmt;
use pg_query::protobuf::ObjectType;
use pg_query::protobuf::SelectStmt;
use pg_query::protobuf::SetOperation;
use pg_query::protobuf::ViewCheckOption;
use pg_query::protobuf::ViewStmt;
use pg_query::Node;
use pg_query::NodeEnum;

#[derive(Debug)]
pub struct RawStmt<'a> {
    pub stmt: &'a NodeEnum,
}

impl Printer {
    pub fn stmt(&mut self, stmt: &RawStmt) {
        match &stmt.stmt {
            NodeEnum::CompositeTypeStmt(stmt) => self.composite_type_stmt(stmt),
            NodeEnum::CreateDomainStmt(stmt) => self.create_domain_stmt(stmt),
            NodeEnum::CreateExtensionStmt(stmt) => self.create_extension_stmt(stmt),
            NodeEnum::CreateForeignTableStmt(stmt) => self.create_foreign_table_stmt(stmt),
            NodeEnum::CreateFunctionStmt(stmt) => self.create_function_stmt(stmt),
            NodeEnum::CreateSchemaStmt(stmt) => self.create_schema_stmt(stmt),
            NodeEnum::CreateStmt(stmt) => self.create_stmt(stmt),
            NodeEnum::CreateTableAsStmt(stmt) => self.create_table_as_stmt(stmt),
            NodeEnum::CreateTrigStmt(stmt) => self.create_trig_stmt(stmt),
            NodeEnum::DefineStmt(stmt) => self.define_stmt(stmt),
            NodeEnum::ExecuteStmt(stmt) => self.execute_stmt(stmt),
            NodeEnum::GrantStmt(stmt) => self.grant_stmt(stmt),
            NodeEnum::IndexStmt(stmt) => self.index_stmt(stmt),
            NodeEnum::SelectStmt(stmt) => self.select_stmt(stmt),
            NodeEnum::ViewStmt(stmt) => self.view_stmt(stmt),
            _ => unimplemented!("{:?}", stmt),
        }
    }

    fn composite_type_stmt(&mut self, stmt: &CompositeTypeStmt) {
        self.word("create type ");
        if let Some(typevar) = &stmt.typevar {
            self.any_name(&make_range_var_into_any_name(typevar));
        }
        self.word(" as ");
        self.word("(");
        self.opt_table_func_element_list(&stmt.coldeflist);
        self.word(")");
    }

    fn create_domain_stmt(&mut self, stmt: &CreateDomainStmt) {
        self.word("create domain ");
        self.any_name(&stmt.domainname);
        self.opt_as();
        if let Some(type_name) = &stmt.type_name {
            self.type_name(type_name);
        }
        self.col_qual_list(&stmt.constraints, stmt.coll_clause.as_deref());
    }

    fn create_extension_stmt(&mut self, stmt: &CreateExtensionStmt) {
        self.word("create extension ");
        self.optional_word("if not exists ", stmt.if_not_exists);
        self.ident(stmt.extname.clone());
        self.nbsp();

        for option in &stmt.options {
            let def_elem = option
                .node
                .as_ref()
                .and_then(|option| match option {
                    NodeEnum::DefElem(def_elem) => Some(def_elem),
                    _ => None,
                })
                .unwrap();

            match def_elem.defname.as_ref() {
                "schema" => {
                    self.word("schema ");
                    self.ident(str_val(&def_elem.arg.clone().unwrap()).unwrap());
                }
                "new_version" => {
                    self.word("version ");
                    self.non_reserved_word_or_scont(
                        str_val(&def_elem.arg.clone().unwrap()).unwrap(),
                    );
                }
                "cascade" => {
                    self.word("cascade ");
                }
                _ => {}
            }
            self.space();
        }
    }

    fn create_foreign_table_stmt(&mut self, stmt: &CreateForeignTableStmt) {
        self.word("create foreign table ");
        self.qualified_name(&Node {
            node: Some(NodeEnum::RangeVar(
                stmt.base_stmt
                    .as_ref()
                    .unwrap()
                    .relation
                    .as_ref()
                    .unwrap()
                    .clone(),
            )),
        });
        self.nbsp();

        if !stmt.base_stmt.as_ref().unwrap().table_elts.is_empty() {
            self.cbox(INDENT);
            self.word("(");
            self.hardbreak_if_nonempty();
            self.print_list(&stmt.base_stmt.as_ref().unwrap().table_elts);
            self.hardbreak();
            self.offset(-INDENT);
            self.end();
            self.word(")");
        }

        self.opt_inherit(&stmt.base_stmt.as_ref().unwrap().inh_relations);

        self.hardbreak();
        self.word("server ");
        self.name(stmt.servername.clone());
        self.nbsp();

        self.create_generic_options(&stmt.options);
    }

    fn create_function_stmt(&mut self, stmt: &CreateFunctionStmt) {
        self.word("create ");
        self.opt_or_replace(stmt.replace);
        self.word("function ");
        self.func_name(&stmt.funcname);
        self.func_args_with_defaults(&stmt.parameters);
        if let Some(return_type) = &stmt.return_type {
            self.word("returns ");
            self.func_return(return_type);
        }
        self.opt_createfunc_opt_list(&stmt.options);
        self.opt_routine_body(stmt.sql_body.as_deref())
    }

    fn create_schema_stmt(&mut self, stmt: &CreateSchemaStmt) {
        self.word("create schema ");
        self.optional_word("if not exists ", stmt.if_not_exists);
        self.ident(stmt.schemaname.clone());

        if let Some(authrole) = &stmt.authrole {
            self.word("authorization ");
            self.role_spec(authrole);
            self.nbsp();
        }

        self.print_list(&stmt.schema_elts);
    }

    fn create_stmt(&mut self, stmt: &CreateStmt) {
        self.cbox(INDENT);
        self.word("create ");

        self.opt_temp(stmt.relation.as_ref().unwrap().relpersistence.clone());

        self.word("table ");

        if stmt.if_not_exists {
            self.word("if not exists ");
        }

        if let Some(relation) = &stmt.relation {
            self.range_var(relation);
            self.nbsp();
        }

        if let Some(of_typename) = &stmt.of_typename {
            self.word("of ");
            self.type_name(of_typename);
            self.nbsp();
        }

        if stmt.partbound.is_some() {
            self.word("partition of ");
            self.range_var(
                stmt.inh_relations
                    .first()
                    .and_then(|node| match node.node.as_ref().unwrap() {
                        NodeEnum::RangeVar(node) => Some(node),
                        _ => None,
                    })
                    .unwrap(),
            );
            self.word(" ");
        }

        if !stmt.table_elts.is_empty() {
            self.word("(");
            self.hardbreak_if_nonempty();
            for (i, elt) in stmt.table_elts.iter().enumerate() {
                self.node(elt);
                if i < stmt.table_elts.len() - 1 {
                    self.word(",");
                }
                self.hardbreak();
            }
            self.offset(-INDENT);
            self.end();
            self.word(")");
        } else if stmt.partbound.is_none() && stmt.of_typename.is_none() {
            self.word("()");
        };

        if let Some(partbound) = &stmt.partbound {
            self.partition_bound_spec(partbound);
            self.word(" ");
        } else {
            self.opt_inherit(&stmt.inh_relations);
        }

        self.opt_with(&stmt.options);

        self.on_commit_action(&stmt.oncommit());

        if !stmt.tablespacename.is_empty() {
            self.word("tablespace ");
            self.ident(stmt.tablespacename.clone());
        }

        self.hardbreak();
    }

    fn create_table_as_stmt(&mut self, stmt: &CreateTableAsStmt) {
        self.word("create ");

        self.opt_temp(
            stmt.into
                .as_ref()
                .unwrap()
                .rel
                .as_ref()
                .unwrap()
                .relpersistence
                .clone(),
        );

        self.object_type(&stmt.objtype());

        if stmt.if_not_exists {
            self.word("if not exists ");
        }

        self.into_clause(stmt.into.as_ref().unwrap());
        self.word(" ");

        self.word("as ");

        if let NodeEnum::SelectStmt(query) = &stmt.query.as_ref().unwrap().node.as_ref().unwrap() {
            self.select_stmt(query);
        }

        self.word(" ");

        if let Some(into) = stmt.into.as_deref() {
            if into.skip_data {
                self.word("with no data ");
            }
        }
    }

    fn create_trig_stmt(&mut self, stmt: &CreateTrigStmt) {
        let mut skip_events_or = true;

        self.word("create ");

        if stmt.replace {
            self.word("or replace ");
        }

        if stmt.isconstraint {
            self.word("constraint ");
        }

        self.word("trigger ");
        self.ident(stmt.trigname.clone());
        self.nbsp();

        match stmt.timing as usize {
            TRIGGER_TYPE_BEFORE => self.word("before "),
            TRIGGER_TYPE_AFTER => self.word("after "),
            TRIGGER_TYPE_INSTEAD => self.word("instead of "),
            _ => {}
        }

        if stmt.events as usize & TRIGGER_TYPE_INSERT != 0 {
            self.word("insert ");
            skip_events_or = false;
        }

        if stmt.events as usize & TRIGGER_TYPE_DELETE != 0 {
            if !skip_events_or {
                self.word("or ");
            }
            self.word("delete ");
            skip_events_or = false;
        }

        if stmt.events as usize & TRIGGER_TYPE_UPDATE != 0 {
            if !skip_events_or {
                self.word("or ");
            }
            self.word("update ");

            if !stmt.columns.is_empty() {
                self.word("of ");
                self.column_list(&stmt.columns);
                self.nbsp();
            }
            skip_events_or = false;
        }

        if stmt.events as usize & TRIGGER_TYPE_TRUNCATE != 0 {
            if !skip_events_or {
                self.word("or ");
            }
            self.word("truncate ");
        }

        self.word("on ");
        self.range_var(stmt.relation.as_ref().unwrap());
        self.nbsp();

        if !stmt.transition_rels.is_empty() {
            self.word("referencing ");
            for transition_rel in &stmt.transition_rels {
                self.node(transition_rel);
                self.nbsp();
            }
        }

        if let Some(constrrel) = &stmt.constrrel {
            self.word("from ");
            self.range_var(constrrel);
            self.nbsp();
        }

        if stmt.deferrable {
            self.word("deferrable ");
        }

        if stmt.initdeferred {
            self.word("initially deferred ");
        }

        if stmt.row {
            self.word("for each row ");
        }

        if let Some(when_clause) = &stmt.when_clause {
            self.word("when (");
            self.node(when_clause);
            self.word(") ");
        }

        self.word("execute function ");
        self.func_name(&stmt.funcname);

        self.word("(");
        for (i, arg) in stmt.args.iter().enumerate() {
            string_literal(self, &str_val(arg).unwrap());
            self.trailing_comma(i >= stmt.args.len() - 1);
        }
        self.word(")");
    }

    fn define_stmt(&mut self, stmt: &DefineStmt) {
        self.cbox(0);
        self.word("create ");

        if stmt.replace {
            self.word("or replace ");
        }

        self.object_type(&stmt.kind());

        if stmt.if_not_exists {
            self.word("if not exists ");
        }

        match stmt.kind() {
            ObjectType::ObjectAggregate => todo!(),
            ObjectType::ObjectOperator => todo!(),
            ObjectType::ObjectType
            | ObjectType::ObjectTsparser
            | ObjectType::ObjectTsdictionary
            | ObjectType::ObjectTstemplate
            | ObjectType::ObjectTsconfiguration
            | ObjectType::ObjectCollation => self.any_name(&stmt.defnames),
            _ => unreachable!(),
        }
        self.space();

        if !stmt.oldstyle && matches!(stmt.kind(), ObjectType::ObjectAggregate) {
            todo!();
        }

        if (matches!(stmt.kind(), ObjectType::ObjectCollation)
            && stmt.definition.len() == 1
            && matches!(
                stmt.definition.first().unwrap().node.as_ref().unwrap(),
                NodeEnum::DefElem(node) if node.defname == "from",
            ))
        {
            self.word("from ");
            todo!();
        } else if !stmt.definition.is_empty() {
            todo!();
        }

        self.end();
    }

    fn execute_stmt(&mut self, stmt: &ExecuteStmt) {
        self.word("execute ");
        self.ident(stmt.name.clone());

        if !stmt.params.is_empty() {
            self.word("(");
            self.print_list(&stmt.params);
            self.word(")");
        }
    }

    fn grant_stmt(&mut self, stmt: &GrantStmt) {
        if stmt.is_grant {
            self.word("grant ");
        } else {
            self.word("revoke ");
        }

        if !stmt.is_grant && stmt.grant_option {
            self.word("grant option for ");
        }

        if !stmt.privileges.is_empty() {
            self.expr_list(&stmt.privileges);
            self.nbsp();
        } else {
            self.word("all ");
        }

        self.word("on ");

        self.privilege_target(&stmt.targtype(), &stmt.objtype(), &stmt.objects);
        self.nbsp();

        if stmt.is_grant {
            self.word("to ");
        } else {
            self.word("from ");
        }

        for (i, grantee) in stmt.grantees.iter().enumerate() {
            self.node(grantee);
            self.trailing_comma(i >= stmt.grantees.len() - 1);
        }

        if stmt.is_grant && stmt.grant_option {
            self.word(" with grant option");
        }

        self.opt_drop_behavior(&stmt.behavior());

        if let Some(grantor) = &stmt.grantor {
            self.word("granted by ");
            self.role_spec(grantor);
        }
    }

    fn index_stmt(&mut self, stmt: &IndexStmt) {
        self.word("create ");

        if stmt.unique {
            self.word("unique ");
        }

        self.word("index ");

        if stmt.concurrent {
            self.word("concurrently ");
        }

        if stmt.if_not_exists {
            self.word("if not exists ");
        }

        self.ident(stmt.idxname.clone());
        self.nbsp();

        self.word("on ");
        self.range_var(stmt.relation.as_ref().unwrap());
        self.nbsp();

        if !&stmt.access_method.is_empty() {
            self.word("using ");
            self.ident(stmt.access_method.clone());
            self.nbsp();
        }

        self.word("(");
        self.print_list(&stmt.index_params);
        self.word(")");

        if !stmt.index_including_params.is_empty() {
            self.word(" include (");
            self.print_list(&stmt.index_including_params);
            self.word(") ");
        }

        if stmt.nulls_not_distinct {
            self.word("nulls not distinct ");
        }

        self.opt_with(&stmt.options);

        if !stmt.table_space.is_empty() {
            self.word("tablespace ");
            self.ident(stmt.table_space.clone());
            self.nbsp();
        }

        self.where_clause(stmt.where_clause.as_deref());
    }

    fn select_stmt(&mut self, stmt: &SelectStmt) {
        if let Some(with_clause) = &stmt.with_clause {
            self.with_clause(with_clause);
            self.word(" ");
        }

        match &stmt.op() {
            SetOperation::SetopNone => {
                if !stmt.values_lists.is_empty() {
                    self.word("values ");

                    for (i, list) in stmt.values_lists.iter().enumerate() {
                        self.word("(");
                        self.print_list(&[list.clone()]);
                        self.word(")");
                        self.trailing_comma(i >= stmt.values_lists.len() - 1);
                    }

                    self.word(" ");
                }

                self.word("select ");

                if !stmt.target_list.is_empty() {
                    if !stmt.distinct_clause.is_empty() {
                        self.word("distinct ");

                        self.word("on (");
                        self.print_list(&stmt.distinct_clause);
                        self.word(") ");
                    }

                    self.print_list(&stmt.target_list);
                    self.word(" ");
                }

                self.from_clause(&stmt.from_clause);
                self.where_clause(stmt.where_clause.as_deref());
            }
            _ => todo!(),
        }
    }

    fn view_stmt(&mut self, stmt: &ViewStmt) {
        self.word("create ");

        if stmt.replace {
            self.word("or replace ");
        }

        self.opt_temp(stmt.view.as_ref().unwrap().relpersistence.clone());

        self.word("view ");
        self.range_var(stmt.view.as_ref().unwrap());

        if !stmt.aliases.is_empty() {
            self.word("(");
            self.column_list(&stmt.aliases);
            self.word(")");
        }

        self.opt_with(&stmt.options);

        self.word(" as ");
        self.node(stmt.query.as_ref().unwrap());
        self.nbsp();

        match stmt.with_check_option() {
            ViewCheckOption::LocalCheckOption => self.word("with local check option "),
            ViewCheckOption::CascadedCheckOption => self.word("with check option "),
            _ => {}
        }
    }
}
