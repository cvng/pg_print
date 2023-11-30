use crate::fmt;
use crate::utils::is_op;
use crate::utils::str_val;
use pg_query::protobuf::AExpr;
use pg_query::protobuf::AExprKind;
use pg_query::Node;

impl fmt::Print for AExpr {
    fn print_in_context(&self, p: &mut fmt::Printer, ctx: &fmt::Context) -> fmt::Result {
        let need_lexpr_parens = false;
        let need_rexpr_parens = false;

        match self.kind() {
            AExprKind::Undefined => todo!(),
            AExprKind::AexprOp => {
                let need_outer_parens = matches!(ctx, fmt::Context::AExpr);

                if need_outer_parens {
                    p.word("(");
                }

                if let Some(lexpr) = &self.lexpr {
                    if need_lexpr_parens {
                        p.word("(");
                    }

                    lexpr.print(p)?;

                    if need_lexpr_parens {
                        p.word(")");
                    }

                    p.nbsp();
                }

                print_qual_op(p, &self.name)?;

                if let Some(rexpr) = &self.rexpr {
                    p.nbsp();

                    if need_rexpr_parens {
                        p.word("(");
                    }

                    rexpr.print(p)?;

                    if need_rexpr_parens {
                        p.word(")");
                    }
                }

                Ok(())
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

fn print_qual_op(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    if list.len() == 1 && is_op(str_val(list.first().unwrap())) {
        p.word(str_val(list.first().unwrap()).unwrap());
    } else {
        p.word("operator(");
        print_any_operator(p, list)?;
        p.word(")");
    }

    Ok(())
}

fn print_any_operator(p: &mut fmt::Printer, list: &[Node]) -> fmt::Result {
    match list.len() {
        2 => {
            p.ident(str_val(list.first().unwrap()).unwrap());
            p.word(".");
            p.word(str_val(list.last().unwrap()).unwrap());
            Ok(())
        }
        1 => {
            p.ident(str_val(list.last().unwrap()).unwrap());
            Ok(())
        }
        _ => Err(fmt::Error),
    }
}
