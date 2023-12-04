use crate::fmt::Printer;
use pg_query::protobuf::RoleSpec;
use pg_query::protobuf::RoleSpecType;

impl Printer {
    pub fn role_spec(&mut self, n: &RoleSpec) {
        match n.roletype() {
            RoleSpecType::RolespecCstring => self.ident(n.rolename.clone()),
            RoleSpecType::RolespecCurrentRole => self.word("current_role"),
            RoleSpecType::RolespecCurrentUser => self.word("current_user"),
            RoleSpecType::RolespecSessionUser => self.word("session_user"),
            RoleSpecType::RolespecPublic => self.word("public"),
            _ => {}
        }
    }
}
