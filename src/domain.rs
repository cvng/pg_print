use crate::fmt::Printer;
use pg_query::protobuf::AlterDomainStmt;
use pg_query::protobuf::CreateDomainStmt;

impl Printer {
    pub fn create_domain_stmt(&mut self, n: &CreateDomainStmt) {
        self.word("create domain ");
        self.any_name(&n.domainname);
        self.opt_as();
        if let Some(type_name) = &n.type_name {
            self.type_name(type_name);
        }
        self.col_qual_list(&n.constraints, n.coll_clause.as_deref());
    }

    pub fn _alter_domain_stmt(&mut self, _n: &AlterDomainStmt) {
        todo!();
    }

    pub fn opt_as(&mut self) {
        self.word(" as ")
    }
}
