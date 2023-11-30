use crate::fmt;
use crate::fmt::gram::print_column_list;
use crate::fmt::gram::print_opt_temp;
use crate::fmt::gram::print_opt_with;
use pg_query::protobuf::ViewCheckOption;
use pg_query::protobuf::ViewStmt;

impl fmt::Print for ViewStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.keyword("create ");

        if self.replace {
            p.keyword("or replace ");
        }

        print_opt_temp(p, self.view.as_ref().unwrap().relpersistence.clone())?;

        p.keyword("view ");
        self.view.as_ref().unwrap().print(p)?;

        if !self.aliases.is_empty() {
            p.word("(");
            print_column_list(p, &self.aliases)?;
            p.word(")");
        }

        print_opt_with(p, &self.options)?;

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
