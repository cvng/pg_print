use crate::cast;
use crate::fmt::Context;
use crate::fmt::Printer;
use pg_query::protobuf::BoolExprType;
use pg_query::protobuf::CollateClause;
use pg_query::protobuf::ConstrType;
use pg_query::protobuf::Constraint;
use pg_query::Node;
use pg_query::NodeEnum;

impl Printer {
    pub fn col_qual_list(&mut self, list: &[Node], col: Option<&CollateClause>) {
        for node in list.iter() {
            if let Some(NodeEnum::Constraint(node)) = &node.node {
                self.col_constraint(node, col)
            }
        }
    }

    fn col_constraint(&mut self, n: &Constraint, _col: Option<&CollateClause>) {
        self.col_constraint_elem(n);
    }

    fn col_constraint_elem(&mut self, n: &Constraint) {
        match n.contype() {
            ConstrType::ConstrCheck => {
                self.word("check ");
                self.word("(");
                let expr_list = &n
                    .raw_expr
                    .as_ref()
                    .and_then(|node| node.node.as_ref())
                    .and_then(|node| cast!(node, NodeEnum::BoolExpr(node)))
                    .map(|expr| expr.args.to_owned())
                    .into_iter()
                    .flatten()
                    .map(|node| node.node)
                    .filter_map(|node| cast!(node, Some(NodeEnum::AExpr(node))))
                    .collect::<Vec<_>>();
                for (i, expr) in expr_list.iter().enumerate() {
                    self.a_expr(expr, &Context::AExpr);

                    match n.raw_expr.as_ref().unwrap().node.as_ref().unwrap() {
                        NodeEnum::BoolExpr(node) => match &node.boolop() {
                            BoolExprType::OrExpr => {
                                if i < expr_list.len() - 1 {
                                    self.word(" or ")
                                }
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                self.word(")");
                self.opt_no_inherit(n.is_no_inherit);
            }
            _ => todo!(),
        }
    }
}
