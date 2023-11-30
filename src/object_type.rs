use crate::fmt;
use pg_query::protobuf::ObjectType;

impl fmt::Print for ObjectType {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        match self {
            ObjectType::ObjectAggregate => p.keyword("aggregate "),
            ObjectType::ObjectOperator => p.keyword("operator "),
            ObjectType::ObjectType => p.keyword("type "),
            ObjectType::ObjectTsparser => p.keyword("text search parser "),
            ObjectType::ObjectTsdictionary => p.keyword("text seach dictionary "),
            ObjectType::ObjectTstemplate => p.keyword("text search template "),
            ObjectType::ObjectTsconfiguration => p.keyword("text search configuration "),
            ObjectType::ObjectCollation => p.keyword("collation "),
            ObjectType::ObjectTable => p.keyword("table "),
            ObjectType::ObjectMatview => p.keyword("materialized view "),
            _ => todo!("{:?}", self),
        }

        Ok(())
    }
}
