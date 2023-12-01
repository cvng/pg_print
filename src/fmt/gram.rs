use super::alg::Printer;
use crate::fmt;
use crate::fmt::Print;
use crate::rel_persistence::RelPersistence;
use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::AConst;
use pg_query::protobuf::DropBehavior;
use pg_query::protobuf::GrantTargetType;
use pg_query::protobuf::ObjectType;
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
            self.comma(i >= list.len() - 1);
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
                self.comma(i >= list.len() - 1);
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
                self.comma(i >= list.len() - 1);
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
            self.comma(i >= list.len() - 1);
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
        todo!("{:?}", list)
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
            todo!("{:?}", list)
        }

        Ok(())
    }

    pub fn signed_iconst(&mut self, node: &Node) {
        self.word(format!("{}", int_val(node).unwrap()));
    }
}
