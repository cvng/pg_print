use crate::fmt;
use crate::utils::print_any_name;
use pg_query::protobuf::DefineStmt;
use pg_query::protobuf::ObjectType;
use pg_query::NodeEnum;

impl fmt::Print for DefineStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        p.cbox(0);
        p.keyword("create ");

        if self.replace {
            p.keyword("or replace ");
        }

        self.kind().print(p)?;

        if self.if_not_exists {
            p.keyword("if not exists ");
        }

        match self.kind() {
            ObjectType::ObjectAggregate => todo!(),
            ObjectType::ObjectOperator => todo!(),
            ObjectType::ObjectType
            | ObjectType::ObjectTsparser
            | ObjectType::ObjectTsdictionary
            | ObjectType::ObjectTstemplate
            | ObjectType::ObjectTsconfiguration
            | ObjectType::ObjectCollation => print_any_name(p, &self.defnames)?,
            _ => unreachable!(),
        }
        p.space();

        if !self.oldstyle && matches!(self.kind(), ObjectType::ObjectAggregate) {
            todo!();
            // p.space();
        }

        if (matches!(self.kind(), ObjectType::ObjectCollation)
            && self.definition.len() == 1
            && matches!(
                self.definition.first().unwrap().node.as_ref().unwrap(),
                NodeEnum::DefElem(node) if node.defname == "from",
            ))
        {
            p.keyword("from ");
            todo!();
        } else if !self.definition.is_empty() {
            todo!()
        }

        p.end();

        Some(())
    }
}
