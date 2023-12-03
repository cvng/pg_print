use crate::fmt;
use pg_query::protobuf::CreateFunctionStmt;

impl fmt::Print for CreateFunctionStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.keyword("create ");
        p.opt_or_replace(self.replace);
        p.keyword("function ");
        p.func_name(&self.funcname)?;
        p.func_args_with_defaults(&self.parameters)?;
        if let Some(return_type) = &self.return_type {
            p.keyword("returns ");
            p.func_return(return_type)?;
        }
        p.opt_createfunc_opt_list(&self.options)?;
        p.opt_routine_body(self.sql_body.as_deref())
    }
}
