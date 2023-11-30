use crate::fmt;
use pg_query::protobuf::ObjectType;

impl fmt::Print for ObjectType {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        match self {
            ObjectType::ObjectAggregate => Ok(p.keyword("aggregate ")),
            ObjectType::ObjectOperator => Ok(p.keyword("operator ")),
            ObjectType::ObjectType => Ok(p.keyword("type ")),
            ObjectType::ObjectTsparser => Ok(p.keyword("text search parser ")),
            ObjectType::ObjectTsdictionary => Ok(p.keyword("text seach dictionary ")),
            ObjectType::ObjectTstemplate => Ok(p.keyword("text search template ")),
            ObjectType::ObjectTsconfiguration => Ok(p.keyword("text search configuration ")),
            ObjectType::ObjectCollation => Ok(p.keyword("collation ")),
            ObjectType::ObjectTable => Ok(p.keyword("table ")),
            ObjectType::ObjectMatview => Ok(p.keyword("materialized view ")),
            _ => unimplemented!("{:?}", self),
        }
    }
}
