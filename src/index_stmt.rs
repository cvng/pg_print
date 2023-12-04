use crate::fmt;
use pg_query::protobuf::IndexStmt;

impl fmt::Print for IndexStmt {
    fn print(&self, p: &mut fmt::Printer) {
        self.word("create ");

        if self.unique {
            self.word("unique ");
        }

        self.word("index ");

        if self.concurrent {
            self.word("concurrently ");
        }

        if self.if_not_exists {
            self.word("if not exists ");
        }

        self.ident(self.idxname.clone());
        self.nbsp();

        self.word("on ");
        self.relation.as_ref().unwrap().print(p);
        self.nbsp();

        if !&self.access_method.is_empty() {
            self.word("using ");
            self.ident(self.access_method.clone());
            self.nbsp();
        }

        self.word("(");
        self.print_list(&self.index_params);
        self.word(")");

        if !self.index_including_params.is_empty() {
            self.word(" include (");
            self.print_list(&self.index_including_params);
            self.word(") ");
        }

        if self.nulls_not_distinct {
            self.word("nulls not distinct ");
        }

        self.opt_with(&self.options);

        if !self.table_space.is_empty() {
            self.word("tablespace ");
            self.ident(self.table_space.clone());
            self.nbsp();
        }

        self.where_clause(self.where_clause.as_deref());
    }
}
