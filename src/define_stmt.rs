use crate::algorithm::Printer;
use crate::create_stmt::node_any_name;
use pg_query::protobuf::DefineStmt;
use pg_query::protobuf::ObjectType;
use pg_query::NodeEnum;

pub fn node_define_stmt(str: &mut Printer, node: &DefineStmt) {
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
