use crate::fmt;
use pg_query::protobuf::RoleSpec;
use pg_query::protobuf::RoleSpecType;

impl fmt::Print for RoleSpec {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        match self.roletype() {
            RoleSpecType::RolespecCstring => p.ident(self.rolename.clone()),
            RoleSpecType::RolespecCurrentRole => p.keyword("current_role"),
            RoleSpecType::RolespecCurrentUser => p.keyword("current_user"),
            RoleSpecType::RolespecSessionUser => p.keyword("session_user"),
            RoleSpecType::RolespecPublic => p.keyword("public"),
            _ => {}
        }

        Ok(())
    }
}
