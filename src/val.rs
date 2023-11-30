use crate::fmt;
use crate::utils::deparse_string_literal;
use pg_query::protobuf::a_const::Val;

impl fmt::Print for Val {
    fn print_in_context(&self, p: &mut fmt::Printer, ctx: &fmt::Context) -> fmt::Result {
        match self {
            Val::Ival(val) => Ok(p.word(format!("{}", val.ival))),
            Val::Fval(val) => Ok(p.word(val.fval.clone())),
            Val::Boolval(val) => Ok(p.word(if val.boolval { "true" } else { "false" })),
            Val::Sval(val) => Ok(match ctx {
                fmt::Context::Identifier => p.ident(val.sval.clone()),
                fmt::Context::Constant => deparse_string_literal(p, &val.sval)?,
                _ => p.word(val.sval.clone()),
            }),
            Val::Bsval(val) => Ok(match val.bsval.chars().next().unwrap() {
                'x' => {
                    p.word("x");
                    deparse_string_literal(p, &val.bsval[1..])?
                }
                'b' => {
                    p.word("b");
                    deparse_string_literal(p, &val.bsval[1..])?
                }
                _ => unreachable!(),
            }),
        }
    }
}

impl fmt::Print for Option<Val> {
    fn print_in_context(&self, p: &mut fmt::Printer, ctx: &fmt::Context) -> fmt::Result {
        match self {
            Some(val) => val.print_in_context(p, ctx),
            None => Ok(p.keyword("null")),
        }
    }
}
