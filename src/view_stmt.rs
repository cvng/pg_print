use crate::fmt;
use pg_query::protobuf::ViewCheckOption;
use pg_query::protobuf::ViewStmt;

impl fmt::Print for ViewStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.keyword("create ");

        if self.replace {
            p.keyword("or replace ");
        }

        p.opt_temp(self.view.as_ref().unwrap().relpersistence.clone())?;

        p.keyword("view ");
        self.view.as_ref().unwrap().print(p)?;

        if !self.aliases.is_empty() {
            p.word("(");
            p.column_list(&self.aliases)?;
            p.word(")");
        }

        p.opt_with(&self.options)?;

        p.keyword(" as ");
        self.query.as_ref().unwrap().print(p)?;
        p.nbsp();

        match self.with_check_option() {
            ViewCheckOption::LocalCheckOption => p.keyword("with local check option "),
            ViewCheckOption::CascadedCheckOption => p.keyword("with check option "),
            _ => {}
        }

        Ok(())
    }
}
