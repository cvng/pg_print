use crate::fmt;
use pg_query::protobuf::WithClause;

impl fmt::Print for WithClause {
    fn print(&self, p: &mut fmt::Printer) {
        p.word("with ");

        if self.recursive {
            p.word("recursive ");
        }

        todo!("{:?}", &self);
    }
}
