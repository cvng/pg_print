use crate::fmt;
use crate::fmt::string_literal;
use crate::fmt::Context;
use crate::fmt::Printer;
use pg_query::protobuf::a_const::Val;

impl Printer {
    pub fn val(&mut self, val: &Val, context: &Context) {
        match val {
            Val::Ival(val) => self.word(format!("{}", val.ival)),
            Val::Fval(val) => self.word(val.fval.clone()),
            Val::Boolval(val) => self.word(if val.boolval { "true" } else { "false" }),
            Val::Sval(val) => match context {
                fmt::Context::Identifier => self.ident(val.sval.clone()),
                fmt::Context::Constant => string_literal(self, &val.sval).unwrap(),
                _ => self.word(val.sval.clone()),
            },
            Val::Bsval(val) => match val.bsval.chars().next().unwrap() {
                'x' => {
                    self.word("x");
                    string_literal(self, &val.bsval[1..]).unwrap()
                }
                'b' => {
                    self.word("b");
                    string_literal(self, &val.bsval[1..]).unwrap()
                }
                _ => unreachable!(),
            },
        }
    }

    pub fn opt_val(&mut self, val: Option<&Val>, context: &Context) {
        match val {
            Some(val) => self.val(val, context),
            None => self.word("null"),
        }
    }
}
