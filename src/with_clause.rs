use crate::fmt::Printer;
use pg_query::protobuf::WithClause;

impl Printer {
    pub fn with_clause(&mut self, n: &WithClause) {
        self.word("with ");

        if n.recursive {
            self.word("recursive ");
        }

        todo!();
    }
}
