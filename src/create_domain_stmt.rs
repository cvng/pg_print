use crate::fmt;
use pg_query::protobuf::CreateDomainStmt;

impl fmt::Print for CreateDomainStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.keyword("create domain ");
        p.any_name(&self.domainname)?;
        p.opt_as();

        if let Some(type_name) = &self.type_name {
            type_name.print(p)?;
            p.nbsp();
        }

        if let Some(coll_clause) = &self.coll_clause {
            coll_clause.print(p)?;
            p.nbsp();
        }

        self.constraints.print(p)
    }
}
