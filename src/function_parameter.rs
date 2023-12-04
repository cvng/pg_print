use crate::fmt::Printer;
use pg_query::protobuf::FunctionParameter;
use pg_query::protobuf::FunctionParameterMode;

impl Printer {
    pub fn function_parameter(&mut self, n: &FunctionParameter) {
        self.arg_class(&n.mode());
        self.param_name(&n.name);
        self.func_type(n.arg_type.as_ref().unwrap());
    }

    pub fn function_parameter_mode(&mut self, n: &FunctionParameterMode) {
        match n {
            FunctionParameterMode::FuncParamIn => self.word("in "),
            FunctionParameterMode::FuncParamOut => self.word("out "),
            FunctionParameterMode::FuncParamInout => self.word("inout "),
            FunctionParameterMode::FuncParamVariadic => self.word("variadic "),
            _ => {}
        }
    }
}
