use crate::fmt::Printer;
use pg_query::protobuf::DefineStmt;
use pg_query::protobuf::ObjectType;
use pg_query::NodeEnum;

impl Printer {
    pub fn define_stmt(&mut self, n: &DefineStmt) {
        self.cbox(0);
        self.word("create ");

        if n.replace {
            self.word("or replace ");
        }

        self.object_type(&n.kind());

        if n.if_not_exists {
            self.word("if not exists ");
        }

        match n.kind() {
            ObjectType::ObjectAggregate => todo!(),
            ObjectType::ObjectOperator => todo!(),
            ObjectType::ObjectType
            | ObjectType::ObjectTsparser
            | ObjectType::ObjectTsdictionary
            | ObjectType::ObjectTstemplate
            | ObjectType::ObjectTsconfiguration
            | ObjectType::ObjectCollation => self.any_name(&n.defnames),
            _ => unreachable!(),
        }
        self.space();

        if !n.oldstyle && matches!(n.kind(), ObjectType::ObjectAggregate) {
            todo!();
            // self.nbsp();
        }

        if (matches!(n.kind(), ObjectType::ObjectCollation)
            && n.definition.len() == 1
            && matches!(
                n.definition.first().unwrap().node.as_ref().unwrap(),
                NodeEnum::DefElem(node) if node.defname == "from",
            ))
        {
            self.word("from ");
            todo!();
        } else if !n.definition.is_empty() {
            todo!();
        }

        self.end();
    }
}
