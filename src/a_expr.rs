use crate::fmt;
use pg_query::protobuf::AExpr;
use pg_query::protobuf::AExprKind;

impl fmt::Print for AExpr {
    fn print_in_context(&self, p: &mut fmt::Printer, ctx: &fmt::Context) -> fmt::Result {
        let need_lexpr_parens = false;
        let need_rexpr_parens = false;

        match self.kind() {
            AExprKind::Undefined => {}
            AExprKind::AexprOp => {
                let need_outer_parens = matches!(ctx, fmt::Context::AExpr);

                p.optional_word("(", need_outer_parens);

                if let Some(lexpr) = &self.lexpr {
                    p.optional_word("(", need_lexpr_parens);
                    lexpr.print(p)?;
                    p.optional_word(")", need_lexpr_parens);
                    p.nbsp();
                }

                p.qual_op(&self.name)?;

                if let Some(rexpr) = &self.rexpr {
                    p.nbsp();
                    p.optional_word("(", need_rexpr_parens);
                    rexpr.print(p)?;
                    p.optional_word(")", need_rexpr_parens);
                }

                p.optional_word(")", need_outer_parens);
            }
            AExprKind::AexprOpAny => todo!("{:?}", self.kind()),
            AExprKind::AexprOpAll => todo!("{:?}", self.kind()),
            AExprKind::AexprDistinct => {
                p.optional_word("(", need_lexpr_parens);
                self.lexpr.as_ref().unwrap().print(p)?;
                p.optional_word(")", need_lexpr_parens);
                p.keyword(" is distinct from ");
                p.optional_word("(", need_rexpr_parens);
                self.rexpr.as_ref().unwrap().print(p)?;
                p.optional_word(")", need_rexpr_parens);
            }
            AExprKind::AexprNotDistinct => todo!("{:?}", self.kind()),
            AExprKind::AexprNullif => todo!("{:?}", self.kind()),
            AExprKind::AexprIn => todo!("{:?}", self.kind()),
            AExprKind::AexprLike => todo!("{:?}", self.kind()),
            AExprKind::AexprIlike => todo!("{:?}", self.kind()),
            AExprKind::AexprSimilar => todo!("{:?}", self.kind()),
            AExprKind::AexprBetween => todo!("{:?}", self.kind()),
            AExprKind::AexprNotBetween => todo!("{:?}", self.kind()),
            AExprKind::AexprBetweenSym => todo!("{:?}", self.kind()),
            AExprKind::AexprNotBetweenSym => todo!("{:?}", self.kind()),
        }

        Ok(())
    }
}
