use crate::fmt;
use pg_query::protobuf::FunctionParameter;
use pg_query::protobuf::FunctionParameterMode;

impl fmt::Print for FunctionParameter {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.arg_class(&self.mode())?;
        p.param_name(&self.name)?;
        p.func_type(self.arg_type.as_ref().unwrap())?;
        Ok(())
    }
}

impl fmt::Print for FunctionParameterMode {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        match self {
            FunctionParameterMode::FuncParamIn => p.word("in "),
            FunctionParameterMode::FuncParamOut => p.word("out "),
            FunctionParameterMode::FuncParamInout => p.word("inout "),
            FunctionParameterMode::FuncParamVariadic => p.word("variadic "),
            _ => {}
        }
        Ok(())
    }
}
