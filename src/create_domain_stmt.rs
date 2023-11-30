use crate::fmt;
use crate::utils::print_any_name;
use crate::utils::print_expr_list;
use pg_query::protobuf::CreateDomainStmt;

impl fmt::Print for CreateDomainStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.keyword("create domain ");
        print_any_name(p, &self.domainname)?;
        p.keyword(" as ");

        self.type_name.as_ref().unwrap().print(p)?;
        p.nbsp();

        if let Some(coll_clause) = &self.coll_clause {
            coll_clause.print(p)?;
            p.nbsp();
        }

        print_expr_list(p, &self.constraints)?;

        Ok(())
    }
}
