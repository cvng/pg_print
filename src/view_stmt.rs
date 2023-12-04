use crate::fmt::Printer;
use pg_query::protobuf::ViewCheckOption;
use pg_query::protobuf::ViewStmt;

impl Printer {
    pub fn view_stmt(&mut self, n: &ViewStmt) {
        self.word("create ");

        if n.replace {
            self.word("or replace ");
        }

        self.opt_temp(n.view.as_ref().unwrap().relpersistence.clone());

        self.word("view ");
        self.range_var(n.view.as_ref().unwrap());

        if !n.aliases.is_empty() {
            self.word("(");
            self.column_list(&n.aliases);
            self.word(")");
        }

        self.opt_with(&n.options);

        self.word(" as ");
        self.node(n.query.as_ref().unwrap());
        self.nbsp();

        match n.with_check_option() {
            ViewCheckOption::LocalCheckOption => self.word("with local check option "),
            ViewCheckOption::CascadedCheckOption => self.word("with check option "),
            _ => {}
        }
    }
}
