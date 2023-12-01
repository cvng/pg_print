use super::utils::int_val;
use super::utils::is_op;
use super::utils::str_val;
use super::utils::string_literal;
use crate::fmt;
use crate::fmt::Print;
use crate::rel_persistence::RelPersistence;
use pg_query::protobuf::DropBehavior;
use pg_query::protobuf::GrantTargetType;
use pg_query::protobuf::ObjectType;
use pg_query::Node;

const NAMEDATALEN: usize = 64;

pub fn expr_list(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    for (i, expr) in list.iter().enumerate() {
        expr.print(p)?;
        p.comma(i >= list.len() - 1);
    }

    Ok(())
}

pub fn column_list(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    for (i, col) in list.iter().enumerate() {
        p.ident(str_val(col).unwrap());

        if i < list.len() - 1 {
            p.word(", ");
        }
    }

    Ok(())
}

pub fn opt_with(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
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

pub fn opt_temp(p: &mut fmt::Printer, relpersistence: String) -> fmt::Result {
    RelPersistence::from(relpersistence).print(p)
}

pub fn any_name(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    for (i, part) in list.iter().enumerate() {
        if i > 0 {
            p.word(".");
        }

        p.ident(str_val(part).unwrap());
    }

    Ok(())
}

pub fn from_clause(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
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

pub fn where_clause(p: &mut fmt::Printer, node: Option<&Node>) -> fmt::Result {
    if let Some(node) = node {
        p.keyword("where ");
        node.print(p)?;
        p.word(" ");
    }

    Ok(())
}

pub fn opt_collate(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    if !list.is_empty() {
        p.keyword("collate ");
        any_name(p, list)?;
        p.nbsp();
    }

    Ok(())
}

pub fn rel_options(_p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    todo!("{:?}", &list);
}

pub fn non_reserved_word_or_scont(p: &mut fmt::Printer, val: String) -> fmt::Result {
    match val.len() {
        0 => p.word("''".to_string()),
        x if x > NAMEDATALEN => string_literal(p, &val)?,
        _ => p.ident(val),
    }

    Ok(())
}

pub fn func_name(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
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

pub fn qual_op(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    if list.len() == 1 && is_op(str_val(list.first().unwrap())) {
        p.word(str_val(list.first().unwrap()).unwrap());
    } else {
        p.word("operator(");
        any_operator(p, list)?;
        p.word(")");
    }

    Ok(())
}

pub fn any_operator(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    match list.len() {
        2 => {
            p.ident(str_val(list.first().unwrap()).unwrap());
            p.word(".");
            p.word(str_val(list.last().unwrap()).unwrap());
            Ok(())
        }
        1 => {
            p.ident(str_val(list.last().unwrap()).unwrap());
            Ok(())
        }
        _ => Err(fmt::Error),
    }
}

pub fn opt_inherit(_p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    if !list.is_empty() {
        todo!("{:?}", list)
    }

    Ok(())
}

pub fn signed_iconst(p: &mut fmt::Printer, node: &Node) {
    p.word(format!("{}", int_val(node).unwrap()));
}

pub fn create_generic_options(_p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    todo!("{:?}", list)
}
