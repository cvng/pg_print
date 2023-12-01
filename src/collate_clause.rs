use crate::fmt;
use pg_query::protobuf::CollateClause;
use pg_query::NodeEnum;

impl fmt::Print for CollateClause {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        if let Some(arg) = &self.arg {
            let need_parens = matches!(arg.node.as_ref().unwrap(), NodeEnum::AExpr(_));

            p.word_if("(", need_parens);
            arg.print(p)?;
            p.word_if(")", need_parens);
            p.nbsp();
        }

        p.keyword("collate ");
        p.any_name(&self.collname)?;

        Ok(())
    }
}
