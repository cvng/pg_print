use crate::fmt::Printer;
use crate::gram::*;
use pg_query::protobuf::CompositeTypeStmt;

impl Printer {
    pub fn composite_type_stmt(&mut self, n: &CompositeTypeStmt) {
        self.word("create type ");
        if let Some(typevar) = &n.typevar {
            self.any_name(&make_range_var_into_any_name(typevar));
        }
        self.word(" as ");
        self.word("(");
        self.opt_table_func_element_list(&n.coldeflist);
        self.word(")");
    }
}
