use crate::fmt;
use crate::fmt::gram;
use pg_query::protobuf::GrantStmt;

impl fmt::Print for GrantStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        if self.is_grant {
            p.keyword("grant ");
        } else {
            p.keyword("revoke ");
        }

        if !self.is_grant && self.grant_option {
            p.keyword("grant option for ");
        }

        if !self.privileges.is_empty() {
            gram::expr_list(p, &self.privileges)?;
            p.nbsp();
        } else {
            p.keyword("all ");
        }

        p.keyword("on ");

        gram::privilege_target(p, &self.targtype(), &self.objtype(), &self.objects)?;
        p.nbsp();

        if self.is_grant {
            p.keyword("to ");
        } else {
            p.keyword("from ");
        }

        for (i, grantee) in self.grantees.iter().enumerate() {
            grantee.print(p)?;
            p.comma(i >= self.grantees.len() - 1);
        }

        if self.is_grant && self.grant_option {
            p.keyword(" with grant option");
        }

        gram::opt_drop_behavior(p, self.behavior())?;

        if let Some(grantor) = &self.grantor {
            p.keyword("granted by ");
            grantor.print(p)?;
        }

        Ok(())
    }
}
