use crate::create_stmt::node_numeric_only;
use crate::fmt;
use crate::fmt::DeparseNodeContext;
use crate::utils::deparse_string_literal;
use pg_query::protobuf::a_const::Val;

impl fmt::Print for Option<Val> {
    fn print_in_context(&self, p: &mut fmt::Printer, ctx: &fmt::Context) -> fmt::Option {
        let Some(val) = self else {
            p.keyword("null");
            return Some(());
        };

        match val {
            Val::Ival(_) | Val::Fval(_) => node_numeric_only(p, val),
            Val::Boolval(val) => p.word(if val.boolval { "true" } else { "false" }),
            Val::Sval(val) => match ctx.context {
                DeparseNodeContext::Identifier => p.ident(val.sval.clone()),
                DeparseNodeContext::Constant => deparse_string_literal(p, &val.sval),
                _ => p.word(val.sval.clone()),
            },
            Val::Bsval(val) => match val.bsval.chars().next().unwrap() {
                'x' => {
                    p.word("x");
                    deparse_string_literal(p, &val.bsval[1..])
                }
                'b' => {
                    p.word("b");
                    deparse_string_literal(p, &val.bsval[1..])
                }
                _ => unreachable!(),
            },
        }

        Some(())
    }
}
