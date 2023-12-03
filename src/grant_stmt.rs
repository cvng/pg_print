use crate::fmt;
use pg_query::protobuf::GrantStmt;

impl fmt::Print for GrantStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        if self.is_grant {
            p.word("grant ");
        } else {
            p.word("revoke ");
        }

        if !self.is_grant && self.grant_option {
            p.word("grant option for ");
        }

        if !self.privileges.is_empty() {
            p.expr_list(&self.privileges)?;
            p.nbsp();
        } else {
            p.word("all ");
        }

        p.word("on ");

        p.privilege_target(&self.targtype(), &self.objtype(), &self.objects)?;
        p.nbsp();

        if self.is_grant {
            p.word("to ");
        } else {
            p.word("from ");
        }

        for (i, grantee) in self.grantees.iter().enumerate() {
            grantee.print(p)?;
            p.trailing_comma(i >= self.grantees.len() - 1);
        }

        if self.is_grant && self.grant_option {
            p.word(" with grant option");
        }

        p.opt_drop_behavior(self.behavior())?;

        if let Some(grantor) = &self.grantor {
            p.word("granted by ");
            grantor.print(p)?;
        }

        Ok(())
    }
}
