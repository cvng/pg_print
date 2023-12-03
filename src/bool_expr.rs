use crate::fmt;
use pg_query::protobuf::BoolExpr;
use pg_query::protobuf::BoolExprType;

impl fmt::Print for BoolExpr {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        match self.boolop() {
            BoolExprType::AndExpr => {
                for (i, arg) in self.args.iter().enumerate() {
                    let need_parens = true; // TODO
                    p.optional_word("(", need_parens);
                    arg.print(p)?;
                    p.optional_word(")", need_parens);
                    p.optional_keyword(" and ", i < self.args.len() - 1);
                }
            }
            BoolExprType::OrExpr => {
                for (i, arg) in self.args.iter().enumerate() {
                    let need_parens = true; // TODO
                    p.optional_word("(", need_parens);
                    arg.print(p)?;
                    p.optional_word(")", need_parens);
                    p.optional_keyword(" or ", i < self.args.len() - 1);
                }
            }
            BoolExprType::NotExpr => {
                let need_parens = true; // TODO
                p.word("not ");
                p.optional_word("(", need_parens);
                self.args.first().unwrap().print(p)?;
                p.optional_word(")", need_parens);
            }
            _ => {}
        }

        Ok(())
    }
}
