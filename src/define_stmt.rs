use crate::fmt;
use pg_query::protobuf::DefineStmt;
use pg_query::protobuf::ObjectType;
use pg_query::NodeEnum;

impl fmt::Print for DefineStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.cbox(0);
        p.word("create ");

        if self.replace {
            p.word("or replace ");
        }

        self.kind().print(p)?;

        if self.if_not_exists {
            p.word("if not exists ");
        }

        match self.kind() {
            ObjectType::ObjectAggregate => todo!("{:?}", self.kind()),
            ObjectType::ObjectOperator => todo!("{:?}", self.kind()),
            ObjectType::ObjectType
            | ObjectType::ObjectTsparser
            | ObjectType::ObjectTsdictionary
            | ObjectType::ObjectTstemplate
            | ObjectType::ObjectTsconfiguration
            | ObjectType::ObjectCollation => p.any_name(&self.defnames)?,
            _ => return Err(fmt::Error),
        }
        p.space();

        if !self.oldstyle && matches!(self.kind(), ObjectType::ObjectAggregate) {
            todo!("{:?}", self.kind());
            // p.nbsp();
        }

        if (matches!(self.kind(), ObjectType::ObjectCollation)
            && self.definition.len() == 1
            && matches!(
                self.definition.first().unwrap().node.as_ref().unwrap(),
                NodeEnum::DefElem(node) if node.defname == "from",
            ))
        {
            p.word("from ");
            todo!("{:?}", self.kind());
        } else if !self.definition.is_empty() {
            todo!("{:?}", self.kind());
        }

        p.end();

        Ok(())
    }
}
