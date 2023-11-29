use crate::create_stmt::node_expr;
use crate::create_stmt::node_qual_op;
use crate::fmt;
use crate::fmt::Context;
use crate::fmt::DeparseNodeContext;
use pg_query::protobuf::AExpr;
use pg_query::protobuf::AExprKind;

impl fmt::Print for AExpr {
    fn print_with_context(&self, p: &mut fmt::Printer, ctx: &Context) -> fmt::Option {
        let need_lexpr_parens = false;
        let need_rexpr_parens = false;

        match self.kind() {
            AExprKind::Undefined => todo!(),
            AExprKind::AexprOp => {
                let need_outer_parens = matches!(ctx.context, DeparseNodeContext::AExpr);

                if need_outer_parens {
                    p.word("(");
                }

                if self.lexpr.is_some() {
                    if need_lexpr_parens {
                        p.word("(");
                    }

                    node_expr(p, self.lexpr.as_deref());

                    if need_lexpr_parens {
                        p.word(")");
                    }

                    p.nbsp();
                }

                node_qual_op(p, &self.name);

                if self.rexpr.is_some() {
                    p.nbsp();

                    if need_rexpr_parens {
                        p.word("(");
                    }

                    node_expr(p, self.rexpr.as_deref());

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
