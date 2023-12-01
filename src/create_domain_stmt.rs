use crate::fmt;
use crate::fmt::gram;
use pg_query::protobuf::CreateDomainStmt;

impl fmt::Print for CreateDomainStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.keyword("create domain ");
        gram::print_any_name(p, &self.domainname)?;
        p.keyword(" as ");

        self.type_name.as_ref().unwrap().print(p)?;
        p.nbsp();

        if let Some(coll_clause) = &self.coll_clause {
            coll_clause.print(p)?;
            p.nbsp();
        }

        self.constraints.print(p)?;

        Ok(())
    }
}
