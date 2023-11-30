use crate::fmt;
use crate::utils::print_expr_list;
use crate::utils::print_opt_with;
use crate::utils::print_where_clause;
use pg_query::protobuf::IndexStmt;

impl fmt::Print for IndexStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.keyword("create ");

        if self.unique {
            p.keyword("unique ");
        }

        p.keyword("index ");

        if self.concurrent {
            p.keyword("concurrently ");
        }

        if self.if_not_exists {
            p.keyword("if not exists ");
        }

        p.ident(self.idxname.clone());
        p.nbsp();

        p.keyword("on ");
        self.relation.as_ref().unwrap().print(p)?;
        p.nbsp();

        if !&self.access_method.is_empty() {
            p.keyword("using ");
            p.ident(self.access_method.clone());
            p.nbsp();
        }

        p.word("(");
        print_expr_list(p, &self.index_params)?;
        p.word(")");

        if !self.index_including_params.is_empty() {
            p.keyword(" include (");
            print_expr_list(p, &self.index_including_params)?;
            p.word(") ");
        }

        if self.nulls_not_distinct {
            p.keyword("nulls not distinct ");
        }

        print_opt_with(p, &self.options)?;

        if !self.table_space.is_empty() {
            p.keyword("tablespace ");
            p.ident(self.table_space.clone());
            p.nbsp();
        }

        print_where_clause(p, self.where_clause.as_deref())?;

        Ok(())
    }
}
