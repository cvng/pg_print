use crate::fmt;
use pg_query::protobuf::FunctionParameter;
use pg_query::protobuf::FunctionParameterMode;

impl fmt::Print for FunctionParameter {
    fn print(&self, p: &mut fmt::Printer) {
        self.arg_class(&self.mode());
        self.param_name(&self.name);
        self.func_type(self.arg_type.as_ref().unwrap());
    }
}

impl fmt::Print for FunctionParameterMode {
    fn print(&self, p: &mut fmt::Printer) {
        match self {
            FunctionParameterMode::FuncParamIn => self.word("in "),
            FunctionParameterMode::FuncParamOut => self.word("out "),
            FunctionParameterMode::FuncParamInout => self.word("inout "),
            FunctionParameterMode::FuncParamVariadic => self.word("variadic "),
            _ => {}
        }
    }
}
