use crate::fmt;
use crate::fmt::gram;
use crate::fmt::gram::str_val;
use pg_query::protobuf::CreateTrigStmt;

const TRIGGER_TYPE_BEFORE: usize = 1 << 1;
const TRIGGER_TYPE_INSERT: usize = 1 << 2;
const TRIGGER_TYPE_DELETE: usize = 1 << 3;
const TRIGGER_TYPE_UPDATE: usize = 1 << 4;
const TRIGGER_TYPE_TRUNCATE: usize = 1 << 5;
const TRIGGER_TYPE_INSTEAD: usize = 1 << 6;
const TRIGGER_TYPE_AFTER: usize = 0;

impl fmt::Print for CreateTrigStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        let mut skip_events_or = true;

        p.keyword("create ");

        if self.replace {
            p.keyword("or replace ");
        }

        if self.isconstraint {
            p.keyword("constraint ");
        }

        p.keyword("trigger ");
        p.ident(self.trigname.clone());
        p.nbsp();

        match self.timing as usize {
            TRIGGER_TYPE_BEFORE => p.keyword("before "),
            TRIGGER_TYPE_AFTER => p.keyword("after "),
            TRIGGER_TYPE_INSTEAD => p.keyword("instead of "),
            _ => {}
        }

        if self.events as usize & TRIGGER_TYPE_INSERT != 0 {
            p.keyword("insert ");
            skip_events_or = false;
        }

        if self.events as usize & TRIGGER_TYPE_DELETE != 0 {
            if !skip_events_or {
                p.keyword("or ");
            }
            p.keyword("delete ");
            skip_events_or = false;
        }

        if self.events as usize & TRIGGER_TYPE_UPDATE != 0 {
            if !skip_events_or {
                p.keyword("or ");
            }
            p.keyword("update ");

            if !self.columns.is_empty() {
                p.keyword("of ");
                gram::print_column_list(p, &self.columns)?;
                p.nbsp();
            }
            skip_events_or = false;
        }

        if self.events as usize & TRIGGER_TYPE_TRUNCATE != 0 {
            if !skip_events_or {
                p.keyword("or ");
            }
            p.keyword("truncate ");
        }

        p.keyword("on ");
        self.relation.as_ref().unwrap().print(p)?;
        p.nbsp();

        if !self.transition_rels.is_empty() {
            p.keyword("referencing ");
            for transition_rel in &self.transition_rels {
                transition_rel.print(p)?;
                p.nbsp();
            }
        }

        if let Some(constrrel) = &self.constrrel {
            p.keyword("from ");
            constrrel.print(p)?;
            p.nbsp();
        }

        if self.deferrable {
            p.keyword("deferrable ");
        }

        if self.initdeferred {
            p.keyword("initially deferred ");
        }

        if self.row {
            p.keyword("for each row ");
        }

        if let Some(when_clause) = &self.when_clause {
            p.keyword("when (");
            when_clause.print(p)?;
            p.word(") ");
        }

        p.keyword("execute function ");
        gram::print_func_name(p, &self.funcname)?;

        p.word("(");
        for (i, arg) in self.args.iter().enumerate() {
            gram::print_string_literal(p, &str_val(arg).unwrap())?;
            p.comma(i >= self.args.len() - 1);
        }
        p.word(")");

        Ok(())
    }
}
