use crate::fmt;
use pg_query::protobuf::RoleSpec;
use pg_query::protobuf::RoleSpecType;

impl fmt::Print for RoleSpec {
    fn print(&self, p: &mut fmt::Printer) {
        match self.roletype() {
            RoleSpecType::RolespecCstring => p.ident(self.rolename.clone()),
            RoleSpecType::RolespecCurrentRole => p.word("current_role"),
            RoleSpecType::RolespecCurrentUser => p.word("current_user"),
            RoleSpecType::RolespecSessionUser => p.word("session_user"),
            RoleSpecType::RolespecPublic => p.word("public"),
            _ => {}
        }
    }
}
