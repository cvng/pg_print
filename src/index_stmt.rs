use crate::fmt::Printer;
use pg_query::protobuf::IndexStmt;

impl Printer {
    fn index_stmt(&mut self, n: &IndexStmt) {
        self.word("create ");

        if n.unique {
            self.word("unique ");
        }

        self.word("index ");

        if n.concurrent {
            self.word("concurrently ");
        }

        if n.if_not_exists {
            self.word("if not exists ");
        }

        self.ident(n.idxname.clone());
        self.nbsp();

        self.word("on ");
        self.range_var(n.relation.as_ref().unwrap());
        self.nbsp();

        if !&n.access_method.is_empty() {
            self.word("using ");
            self.ident(n.access_method.clone());
            self.nbsp();
        }

        self.word("(");
        self.print_list(&n.index_params);
        self.word(")");

        if !n.index_including_params.is_empty() {
            self.word(" include (");
            self.print_list(&n.index_including_params);
            self.word(") ");
        }

        if n.nulls_not_distinct {
            self.word("nulls not distinct ");
        }

        self.opt_with(&n.options);

        if !n.table_space.is_empty() {
            self.word("tablespace ");
            self.ident(n.table_space.clone());
            self.nbsp();
        }

        self.where_clause(n.where_clause.as_deref());
    }
}
