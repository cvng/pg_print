use crate::fmt::Context;
use crate::fmt::Printer;
use pg_query::protobuf::AExpr;
use pg_query::protobuf::AExprKind;
use pg_query::protobuf::BoolExpr;
use pg_query::protobuf::BoolExprType;

impl Printer {
    pub fn a_expr(&mut self, n: &AExpr, context: &Context) {
        let need_lexpr_parens = false;
        let need_rexpr_parens = false;

        match n.kind() {
            AExprKind::Undefined => {}
            AExprKind::AexprOp => {
                let need_outer_parens = matches!(context, Context::AExpr);

                self.optional_word("(", need_outer_parens);

                if let Some(lexpr) = &n.lexpr {
                    self.optional_word("(", need_lexpr_parens);
                    self.node(lexpr);
                    self.optional_word(")", need_lexpr_parens);
                    self.nbsp();
                }

                self.qual_op(&n.name).unwrap();

                if let Some(rexpr) = &n.rexpr {
                    self.nbsp();
                    self.optional_word("(", need_rexpr_parens);
                    self.node(rexpr);
                    self.optional_word(")", need_rexpr_parens);
                }

                self.optional_word(")", need_outer_parens);
            }
            AExprKind::AexprOpAny => todo!(),
            AExprKind::AexprOpAll => todo!(),
            AExprKind::AexprDistinct => {
                self.optional_word("(", need_lexpr_parens);
                self.node(n.lexpr.as_ref().unwrap());
                self.optional_word(")", need_lexpr_parens);
                self.word(" is distinct from ");
                self.optional_word("(", need_rexpr_parens);
                self.node(n.rexpr.as_ref().unwrap());
                self.optional_word(")", need_rexpr_parens);
            }
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

    pub fn _b_expr(&mut self) {
        todo!()
    }

    pub fn _c_expr(&mut self) {
        todo!()
    }

    pub fn bool_expr(&mut self, n: &BoolExpr) {
        match n.boolop() {
            BoolExprType::AndExpr => {
                for (i, arg) in n.args.iter().enumerate() {
                    let need_parens = true; // TODO
                    self.optional_word("(", need_parens);
                    self.node(arg);
                    self.optional_word(")", need_parens);
                    self.optional_word(" and ", i < n.args.len() - 1);
                }
            }
            BoolExprType::OrExpr => {
                for (i, arg) in n.args.iter().enumerate() {
                    let need_parens = true; // TODO
                    self.optional_word("(", need_parens);
                    self.node(arg);
                    self.optional_word(")", need_parens);
                    self.optional_word(" or ", i < n.args.len() - 1);
                }
            }
            BoolExprType::NotExpr => {
                let need_parens = true; // TODO
                self.word("not ");
                self.optional_word("(", need_parens);
                self.node(n.args.first().unwrap());
                self.optional_word(")", need_parens);
            }
            _ => {}
        }
    }
}
