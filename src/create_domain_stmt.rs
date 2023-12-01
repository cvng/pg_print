use crate::fmt;
use pg_query::protobuf::CreateDomainStmt;

impl fmt::Print for CreateDomainStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.keyword("create domain ");
        p.any_name(&self.domainname)?;
        p.opt_as();
        self.type_name
            .as_ref()
            .and_then(|t| t.print(p).ok())
            .and_then(|_| p.nbsp());
        self.coll_clause
            .as_ref()
            .and_then(|c| c.print(p).ok())
            .and_then(|_| p.nbsp());
        self.constraints.print(p)
    }
}
