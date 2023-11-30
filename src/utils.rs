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

const ESCAPE_STRING_SYNTAX: char = 'E';

const NAMEDATALEN: usize = 64;

// See https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c#L53.
pub fn print_string_literal(p: &mut fmt::Printer, val: &str) -> fmt::Result {
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

pub fn print_expr_list(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    for (i, expr) in list.iter().enumerate() {
        expr.print(p)?;
        p.comma(i >= list.len() - 1);
    }

    Ok(())
}

pub fn print_column_list(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    for (i, col) in list.iter().enumerate() {
        p.ident(str_val(col).unwrap());

        if i < list.len() - 1 {
            p.word(", ");
        }
    }

    Ok(())
}

pub fn print_opt_with(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    if !list.is_empty() {
        p.keyword(" with ");
        p.word("(");

        for (i, option) in list.iter().enumerate() {
            option.print(p)?;
            p.comma(i >= list.len() - 1);
        }

        p.word(")");
        p.nbsp();
    }

    Ok(())
}

pub fn print_opt_temp(p: &mut fmt::Printer, relpersistence: String) -> fmt::Result {
    RelPersistence::from(relpersistence).print(p)
}

pub fn print_any_name(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    for (i, part) in list.iter().enumerate() {
        if i > 0 {
            p.word(".");
        }

        p.ident(str_val(part).unwrap());
    }

    Ok(())
}

pub fn print_from_clause(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    if !list.is_empty() {
        p.keyword("from ");

        for (i, item) in list.iter().enumerate() {
            item.print(p)?;
            p.comma(i >= list.len() - 1);
        }
        p.word(" ");
    }

    Ok(())
}

pub fn print_where_clause(p: &mut fmt::Printer, node: Option<&Node>) -> fmt::Result {
    if let Some(node) = node {
        p.keyword("where ");
        node.print(p)?;
        p.word(" ");
    }

    Ok(())
}

pub fn print_opt_collate(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    if !list.is_empty() {
        p.keyword("collate ");
        print_any_name(p, list)?;
        p.nbsp();
    }

    Ok(())
}

pub fn print_rel_options(_p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    todo!("{:?}", &list);
}

pub fn print_non_reserved_word_or_scont(p: &mut fmt::Printer, val: String) -> fmt::Result {
    match val.len() {
        0 => p.word("''".to_string()),
        x if x > NAMEDATALEN => print_string_literal(p, &val)?,
        _ => p.ident(val),
    }

    Ok(())
}

pub fn print_func_name(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    for (i, part) in list.iter().enumerate() {
        if i > 0 {
            p.word(".");
        }

        p.ident(str_val(part).unwrap());
    }

    Ok(())
}

pub fn privilege_target(
    p: &mut fmt::Printer,
    targtype: &GrantTargetType,
    objtype: &ObjectType,
    objs: &[Node],
) -> fmt::Result {
    match targtype {
        GrantTargetType::AclTargetObject => match objtype {
            ObjectType::ObjectTable => objs.print(p)?,
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
                p.keyword("schema ");
                name_list(p, objs)?;
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

pub fn opt_drop_behavior(p: &mut fmt::Printer, node: DropBehavior) -> fmt::Result {
    match node {
        DropBehavior::DropRestrict => {}
        DropBehavior::DropCascade => p.keyword("cascade "),
        _ => {}
    };

    Ok(())
}

pub fn name_list(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    for (i, name) in list.iter().enumerate() {
        p.ident(str_val(name).unwrap());
        p.comma(i >= list.len() - 1);
    }

    Ok(())
}
