use super::algo::Printer;
use crate::fmt;
use crate::fmt::Print;
use crate::rel_persistence::RelPersistence;
use crate::INDENT;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::AConst;
use pg_query::protobuf::DropBehavior;
use pg_query::protobuf::FunctionParameterMode;
use pg_query::protobuf::GrantTargetType;
use pg_query::protobuf::List;
use pg_query::protobuf::ObjectType;
use pg_query::protobuf::TypeName;
use pg_query::Node;
use pg_query::NodeEnum;

const NAMEDATALEN: usize = 64;
const ESCAPE_STRING_SYNTAX: char = 'E';

// See https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c#L53.
pub fn string_literal(p: &mut Printer, val: &str) -> fmt::Result {
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

    Ok(())
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

impl Printer {
    pub fn any_name(&mut self, list: &[Node]) -> fmt::Result {
        for (i, part) in list.iter().enumerate() {
            if i > 0 {
                self.word(".");
            }

            self.ident(str_val(part).unwrap());
        }

        Ok(())
    }

    pub fn opt_as(&mut self) {
        self.keyword(" as ")
    }

    pub fn expr_list(&mut self, list: &[Node]) -> fmt::Result {
        for (i, expr) in list.iter().enumerate() {
            expr.print(self)?;
            self.trailing_comma(i >= list.len() - 1);
        }

        Ok(())
    }

    pub fn column_list(&mut self, list: &[Node]) -> fmt::Result {
        for (i, col) in list.iter().enumerate() {
            self.ident(str_val(col).unwrap());

            if i < list.len() - 1 {
                self.word(", ");
            }
        }

        Ok(())
    }

    pub fn opt_with(&mut self, list: &[Node]) -> fmt::Result {
        if !list.is_empty() {
            self.keyword(" with ");
            self.word("(");

            for (i, option) in list.iter().enumerate() {
                option.print(self)?;
                self.trailing_comma(i >= list.len() - 1);
            }

            self.word(")");
            self.nbsp();
        }

        Ok(())
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_clause(&mut self, list: &[Node]) -> fmt::Result {
        if !list.is_empty() {
            self.keyword("from ");

            for (i, item) in list.iter().enumerate() {
                item.print(self)?;
                self.trailing_comma(i >= list.len() - 1);
            }
            self.word(" ");
        }

        Ok(())
    }

    pub fn where_clause(&mut self, node: Option<&Node>) -> fmt::Result {
        if let Some(node) = node {
            self.keyword("where ");
            node.print(self)?;
            self.word(" ");
        }

        Ok(())
    }

    pub fn opt_collate(&mut self, list: &[Node]) -> fmt::Result {
        if !list.is_empty() {
            self.keyword("collate ");
            self.any_name(list)?;
            self.nbsp();
        }

        Ok(())
    }

    pub fn reloptions(&mut self, list: &[Node]) -> fmt::Result {
        todo!("{:?}", &list);
    }

    pub fn func_name(&mut self, list: &[Node]) -> fmt::Result {
        for (i, part) in list.iter().enumerate() {
            if i > 0 {
                self.word(".");
            }

            self.ident(str_val(part).unwrap());
        }

        Ok(())
    }

    pub fn func_args_with_defaults(&mut self, list: &[Node]) -> fmt::Result {
        if !list.is_empty() {
            self.cbox(INDENT);
            self.word("(");
            self.hardbreak_if_nonempty();
            for (i, arg) in list.iter().enumerate() {
                arg.print(self)?;
                self.trailing_comma(i >= list.len() - 1);
            }
            self.space();
            self.offset(-INDENT);
            self.end();
            self.word(")");
            self.nbsp();
        }
        Ok(())
    }

    pub fn func_return(&mut self, node: &TypeName) -> fmt::Result {
        node.print(self)?;
        self.nbsp();

        Ok(())
    }

    pub fn opt_createfunc_opt_list(&mut self, list: &[Node]) -> fmt::Result {
        if !list.is_empty() {
            for option in list.iter().skip(1) {
                if let NodeEnum::DefElem(node) = option.node.as_ref().unwrap() {
                    if let Some(arg) = &node.arg {
                        if node.defname == "volatility" && str_val(arg).unwrap() == "stable" {
                        } else {
                            self.keyword(node.defname.clone());
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
                            self.keyword(node.defname.clone());
                            self.cbox(INDENT);
                            self.nbsp();
                            self.word("$$");
                            self.hardbreak_if_nonempty();
                            if let NodeEnum::List(List { items }) = arg.node.as_ref().unwrap() {
                                self.word(
                                    str_val(items.first().unwrap())
                                        .unwrap()
                                        .trim_start()
                                        .to_owned(),
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
        Ok(())
    }

    pub fn opt_routine_body(&mut self, node: Option<&Node>) -> fmt::Result {
        if let Some(node) = node {
            self.keyword("as ");
            node.print(self)?;
            self.nbsp();
        }

        Ok(())
    }

    pub fn arg_class(&mut self, node: &FunctionParameterMode) -> fmt::Result {
        node.print(self)
    }

    pub fn param_name(&mut self, val: &str) -> fmt::Result {
        self.ident(val.to_string());
        self.word(" ");
        Ok(())
    }

    pub fn func_type(&mut self, node: &TypeName) -> fmt::Result {
        node.print(self)?;
        Ok(())
    }

    pub fn privilege_target(
        &mut self,
        targtype: &GrantTargetType,
        objtype: &ObjectType,
        objs: &[Node],
    ) -> fmt::Result {
        match targtype {
            GrantTargetType::AclTargetObject => match objtype {
                ObjectType::ObjectTable => objs.print(self)?,
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
                    self.keyword("schema ");
                    self.name_list(objs)?;
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

        Ok(())
    }

    pub fn opt_drop_behavior(&mut self, node: DropBehavior) -> fmt::Result {
        match node {
            DropBehavior::DropRestrict => {}
            DropBehavior::DropCascade => self.keyword("cascade "),
            _ => {}
        };

        Ok(())
    }

    pub fn name_list(&mut self, list: &[Node]) -> fmt::Result {
        for (i, name) in list.iter().enumerate() {
            self.ident(str_val(name).unwrap());
            self.trailing_comma(i >= list.len() - 1);
        }

        Ok(())
    }

    pub fn qual_op(&mut self, list: &[Node]) -> fmt::Result {
        if list.len() == 1 && is_op(str_val(list.first().unwrap())) {
            self.word(str_val(list.first().unwrap()).unwrap());
        } else {
            self.word("operator(");
            self.any_operator(list)?;
            self.word(")");
        }

        Ok(())
    }

    pub fn any_operator(&mut self, list: &[Node]) -> fmt::Result {
        match list.len() {
            2 => {
                self.ident(str_val(list.first().unwrap()).unwrap());
                self.word(".");
                self.word(str_val(list.last().unwrap()).unwrap());
                Ok(())
            }
            1 => {
                self.ident(str_val(list.last().unwrap()).unwrap());
                Ok(())
            }
            _ => Err(fmt::Error),
        }
    }

    pub fn create_generic_options(&mut self, list: &[Node]) -> fmt::Result {
        if !list.is_empty() {
            self.keyword("options ");
            self.word("(");
            self.generic_option_list(list)?;
            self.word(") ");
        }
        Ok(())
    }

    pub fn generic_option_list(&mut self, list: &[Node]) -> fmt::Result {
        self.qualified_name_list(list)
    }

    pub fn opt_temp(&mut self, relpersistence: String) -> fmt::Result {
        RelPersistence::from(relpersistence).print(self)
    }

    pub fn non_reserved_word_or_scont(&mut self, val: String) -> fmt::Result {
        match val.len() {
            0 => self.word("''".to_string()),
            x if x > NAMEDATALEN => string_literal(self, &val)?,
            _ => self.ident(val),
        }

        Ok(())
    }

    pub fn opt_inherit(&mut self, list: &[Node]) -> fmt::Result {
        if !list.is_empty() {
            self.keyword("inherits ");
            self.word("(");
            self.qualified_name_list(list)?;
            self.word(") ");
        }
        Ok(())
    }

    pub fn signed_iconst(&mut self, node: &Node) {
        self.word(format!("{}", int_val(node).unwrap()));
    }

    pub fn qualified_name_list(&mut self, list: &[Node]) -> fmt::Result {
        for (i, name) in list.iter().enumerate() {
            self.qualified_name(name)?;
            if i < list.len() - 1 {
                self.word(",");
                self.nbsp();
            }
        }
        Ok(())
    }

    pub fn qualified_name(&mut self, node: &Node) -> fmt::Result {
        node.print(self)
    }

    pub fn name(&mut self, name: String) {
        self.ident(name);
    }

    pub fn opt_or_replace(&mut self, replace: bool) {
        if replace {
            self.keyword("or replace ");
        }
    }
}
