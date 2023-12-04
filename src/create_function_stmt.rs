use crate::fmt::Printer;
use pg_query::protobuf::CreateFunctionStmt;

impl Printer {
    pub fn create_function_stmt(&mut self, n: &CreateFunctionStmt) {
        self.word("create ");
        self.opt_or_replace(n.replace);
        self.word("function ");
        self.func_name(&n.funcname);
        self.func_args_with_defaults(&n.parameters);
        if let Some(return_type) = &n.return_type {
            self.word("returns ");
            self.func_return(return_type);
        }
        self.opt_createfunc_opt_list(&n.options);
        self.opt_routine_body(n.sql_body.as_deref())
    }
}
