use crate::fmt::Printer;
use pg_query::protobuf::ObjectType;

impl Printer {
    pub fn object_type(&self, n: &ObjectType) {
        match n {
            ObjectType::ObjectAggregate => self.word("aggregate "),
            ObjectType::ObjectOperator => self.word("operator "),
            ObjectType::ObjectType => self.word("type "),
            ObjectType::ObjectTsparser => self.word("text search parser "),
            ObjectType::ObjectTsdictionary => self.word("text seach dictionary "),
            ObjectType::ObjectTstemplate => self.word("text search template "),
            ObjectType::ObjectTsconfiguration => self.word("text search configuration "),
            ObjectType::ObjectCollation => self.word("collation "),
            ObjectType::ObjectTable => self.word("table "),
            ObjectType::ObjectMatview => self.word("materialized view "),
            _ => todo!(),
        }
    }
}
