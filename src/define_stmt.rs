use crate::fmt;
use pg_query::protobuf::DefineStmt;
use pg_query::protobuf::ObjectType;
use pg_query::NodeEnum;

impl fmt::Print for DefineStmt {
    fn print(&self, p: &mut fmt::Printer) {
        self.cbox(0);
        self.word("create ");

        if self.replace {
            self.word("or replace ");
        }

        self.kind().print(p);

        if self.if_not_exists {
            self.word("if not exists ");
        }

        match self.kind() {
            ObjectType::ObjectAggregate => todo!("{:?}", self.kind()),
            ObjectType::ObjectOperator => todo!("{:?}", self.kind()),
            ObjectType::ObjectType
            | ObjectType::ObjectTsparser
            | ObjectType::ObjectTsdictionary
            | ObjectType::ObjectTstemplate
            | ObjectType::ObjectTsconfiguration
            | ObjectType::ObjectCollation => self.any_name(&self.defnames),
            _ => unreachable!(),
        }
        self.space();

        if !self.oldstyle && matches!(self.kind(), ObjectType::ObjectAggregate) {
            todo!("{:?}", self.kind());
            // self.nbsp();
        }

        if (matches!(self.kind(), ObjectType::ObjectCollation)
            && self.definition.len() == 1
            && matches!(
                self.definition.first().unwrap().node.as_ref().unwrap(),
                NodeEnum::DefElem(node) if node.defname == "from",
            ))
        {
            self.word("from ");
            todo!("{:?}", self.kind());
        } else if !self.definition.is_empty() {
            todo!("{:?}", self.kind());
        }

        self.end();
    }
}
