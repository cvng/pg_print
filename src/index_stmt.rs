use crate::fmt;
use pg_query::protobuf::IndexStmt;

impl fmt::Print for IndexStmt {
    fn print(&self, p: &mut fmt::Printer) {
        p.word("create ");

        if self.unique {
            p.word("unique ");
        }

        p.word("index ");

        if self.concurrent {
            p.word("concurrently ");
        }

        if self.if_not_exists {
            p.word("if not exists ");
        }

        p.ident(self.idxname.clone());
        p.nbsp();

        p.word("on ");
        self.relation.as_ref().unwrap().print(p);
        p.nbsp();

        if !&self.access_method.is_empty() {
            p.word("using ");
            p.ident(self.access_method.clone());
            p.nbsp();
        }

        p.word("(");
        p.print_list(&self.index_params);
        p.word(")");

        if !self.index_including_params.is_empty() {
            p.word(" include (");
            p.print_list(&self.index_including_params);
            p.word(") ");
        }

        if self.nulls_not_distinct {
            p.word("nulls not distinct ");
        }

        p.opt_with(&self.options);

        if !self.table_space.is_empty() {
            p.word("tablespace ");
            p.ident(self.table_space.clone());
            p.nbsp();
        }

        p.where_clause(self.where_clause.as_deref());
    }
}
