use crate::algo::Printer;
use crate::conv::Context;
use pg_query::protobuf::AExpr;
use pg_query::protobuf::AExprKind;
use pg_query::protobuf::BoolExpr;
use pg_query::protobuf::BoolExprType;
use pg_query::Node;
use pg_query::NodeEnum;

impl Printer {
    pub fn a_expr(&mut self, n: &AExpr, _context: &Context) {
        let need_lexpr_parens = false;
        let need_rexpr_parens = false;

        match n.kind() {
            AExprKind::Undefined => {}
            AExprKind::AexprOp => {
                let need_outer_parens = false; // TODO: matches!(context, Context::AExpr);

                self.optional_word("(", need_outer_parens);

                if let Some(lexpr) = &n.lexpr {
                    self.optional_word("(", need_lexpr_parens);
                    self.node(lexpr);
                    self.optional_word(")", need_lexpr_parens);
                    self.nbsp();
                }

                self.qual_op(&n.name);

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

    pub fn node(&mut self, n: &Node) {
        if let Some(node) = &n.node {
            match node {
                NodeEnum::RangeVar(n) => self.range_var(n),
                NodeEnum::BoolExpr(n) => self.bool_expr(n),
                NodeEnum::AExpr(n) => self.a_expr(n, &Context::None),
                NodeEnum::ColumnRef(n) => self.column_ref(n),
                NodeEnum::ResTarget(n) => self.res_target(n),
                NodeEnum::ColumnDef(n) => self.column_def(n),
                NodeEnum::IndexElem(n) => self.index_elem(n),
                NodeEnum::Constraint(n) => self.constraint(n),
                NodeEnum::DefElem(n) => self.def_elem(n),
                NodeEnum::AccessPriv(n) => self.access_priv(n),
                NodeEnum::FunctionParameter(n) => self.function_parameter(n),
                NodeEnum::RoleSpec(n) => self.role_spec(n),
                NodeEnum::Integer(n) => self.integer(n),
                NodeEnum::String(n) => self.string(n),
                NodeEnum::List(n) => self.list(n),
                NodeEnum::AConst(n) => self.a_const(n),
                _ => unimplemented!("{:?}", &n),
            }
        }
    }
}
