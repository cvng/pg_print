use crate::fmt;
use crate::fmt::Print;
use crate::fmt::Printer;
use pg_query::protobuf::CreateDomainStmt;

impl Printer {
    pub fn create_domain_stmt(&mut self, n: &CreateDomainStmt) -> fmt::Result {
        self.word("create domain ");
        self.any_name(&n.domainname)?;
        self.opt_as();

        if let Some(type_name) = &n.type_name {
            type_name.print(self)?;
            self.nbsp();
        }

        if let Some(coll_clause) = &n.coll_clause {
            coll_clause.print(self)?;
            self.nbsp();
        }

        n.constraints.print(self)
    }

    pub fn opt_as(&mut self) {
        self.word(" as ")
    }
}
