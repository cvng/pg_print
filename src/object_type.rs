use crate::fmt;
use pg_query::protobuf::ObjectType;

impl fmt::Print for ObjectType {
    fn print(&self, p: &mut fmt::Printer) {
        match self {
            ObjectType::ObjectAggregate => p.word("aggregate "),
            ObjectType::ObjectOperator => p.word("operator "),
            ObjectType::ObjectType => p.word("type "),
            ObjectType::ObjectTsparser => p.word("text search parser "),
            ObjectType::ObjectTsdictionary => p.word("text seach dictionary "),
            ObjectType::ObjectTstemplate => p.word("text search template "),
            ObjectType::ObjectTsconfiguration => p.word("text search configuration "),
            ObjectType::ObjectCollation => p.word("collation "),
            ObjectType::ObjectTable => p.word("table "),
            ObjectType::ObjectMatview => p.word("materialized view "),
            _ => todo!("{:?}", self),
        }
    }
}
