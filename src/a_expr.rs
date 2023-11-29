use crate::fmt;
use crate::utils::is_op;
use crate::utils::str_val;
use pg_query::protobuf::AExpr;
use pg_query::protobuf::AExprKind;
use pg_query::Node;

impl fmt::Print for AExpr {
    fn print_in_context(&self, p: &mut fmt::Printer, ctx: &fmt::Context) -> fmt::Option {
        let need_lexpr_parens = false;
        let need_rexpr_parens = false;

        match self.kind() {
            AExprKind::Undefined => todo!(),
            AExprKind::AexprOp => {
                let need_outer_parens = matches!(ctx, fmt::Context::AExpr);

                if need_outer_parens {
                    p.word("(");
                }

                if self.lexpr.is_some() {
                    if need_lexpr_parens {
                        p.word("(");
                    }

                    self.lexpr.as_deref()?.print(p)?;

                    if need_lexpr_parens {
                        p.word(")");
                    }

                    p.nbsp();
                }

                print_qual_op(p, &self.name);

                if self.rexpr.is_some() {
                    p.nbsp();

                    if need_rexpr_parens {
                        p.word("(");
                    }

                    self.rexpr.as_deref()?.print(p)?;

                    if need_rexpr_parens {
                        p.word(")");
                    }
                }

                Some(())
            }
            AExprKind::AexprOpAny => todo!(),
            AExprKind::AexprOpAll => todo!(),
            AExprKind::AexprDistinct => todo!(),
            AExprKind::AexprNotDistinct => todo!(),
            AExprKind::AexprNullif => todo!(),
            AExprKind::AexprIn => todo!(),
            AExprKind::AexprLike => todo!(),
            AExprKind::AexprIlike => todo!(),
            AExprKind::AexprSimilar => todo!(),
            AExprKind::AexprBetween => todo!(),
            AExprKind::AexprNotBetween => todo!(),
            AExprKind::AexprBetweenSym => todo!(),
            AExprKind::AexprNotBetweenSym => todo!(),
        }
    }
}

fn print_qual_op(str: &mut fmt::Printer, list: &[Node]) {
    if list.len() == 1 && is_op(str_val(list.first().unwrap())) {
        str.word(str_val(list.first().unwrap()).unwrap());
    } else {
        str.word("operator(");
        print_any_operator(str, list);
        str.word(")");
    }
}

fn print_any_operator(str: &mut fmt::Printer, list: &[Node]) {
    match list.len() {
        2 => {
            str.ident(str_val(list.first().unwrap()).unwrap());
            str.word(".");
            str.word(str_val(list.last().unwrap()).unwrap());
        }
        1 => str.ident(str_val(list.last().unwrap()).unwrap()),
        _ => unreachable!(),
    }
}
