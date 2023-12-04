use crate::fmt;
use crate::fmt::str_val;
use crate::fmt::string_literal;
use pg_query::protobuf::CreateTrigStmt;

const TRIGGER_TYPE_BEFORE: usize = 1 << 1;
const TRIGGER_TYPE_INSERT: usize = 1 << 2;
const TRIGGER_TYPE_DELETE: usize = 1 << 3;
const TRIGGER_TYPE_UPDATE: usize = 1 << 4;
const TRIGGER_TYPE_TRUNCATE: usize = 1 << 5;
const TRIGGER_TYPE_INSTEAD: usize = 1 << 6;
const TRIGGER_TYPE_AFTER: usize = 0;

impl fmt::Print for CreateTrigStmt {
    fn print(&self, p: &mut fmt::Printer) {
        let mut skip_events_or = true;

        p.word("create ");

        if self.replace {
            p.word("or replace ");
        }

        if self.isconstraint {
            p.word("constraint ");
        }

        p.word("trigger ");
        p.ident(self.trigname.clone());
        p.nbsp();

        match self.timing as usize {
            TRIGGER_TYPE_BEFORE => p.word("before "),
            TRIGGER_TYPE_AFTER => p.word("after "),
            TRIGGER_TYPE_INSTEAD => p.word("instead of "),
            _ => {}
        }

        if self.events as usize & TRIGGER_TYPE_INSERT != 0 {
            p.word("insert ");
            skip_events_or = false;
        }

        if self.events as usize & TRIGGER_TYPE_DELETE != 0 {
            if !skip_events_or {
                p.word("or ");
            }
            p.word("delete ");
            skip_events_or = false;
        }

        if self.events as usize & TRIGGER_TYPE_UPDATE != 0 {
            if !skip_events_or {
                p.word("or ");
            }
            p.word("update ");

            if !self.columns.is_empty() {
                p.word("of ");
                p.column_list(&self.columns);
                p.nbsp();
            }
            skip_events_or = false;
        }

        if self.events as usize & TRIGGER_TYPE_TRUNCATE != 0 {
            if !skip_events_or {
                p.word("or ");
            }
            p.word("truncate ");
        }

        p.word("on ");
        self.relation.as_ref().unwrap().print(p);
        p.nbsp();

        if !self.transition_rels.is_empty() {
            p.word("referencing ");
            for transition_rel in &self.transition_rels {
                p.node(transition_rel);
                p.nbsp();
            }
        }

        if let Some(constrrel) = &self.constrrel {
            p.word("from ");
            constrrel.print(p);
            p.nbsp();
        }

        if self.deferrable {
            p.word("deferrable ");
        }

        if self.initdeferred {
            p.word("initially deferred ");
        }

        if self.row {
            p.word("for each row ");
        }

        if let Some(when_clause) = &self.when_clause {
            p.word("when (");
            p.node(when_clause);
            p.word(") ");
        }

        p.word("execute function ");
        p.func_name(&self.funcname);

        p.word("(");
        for (i, arg) in self.args.iter().enumerate() {
            string_literal(p, &str_val(arg).unwrap());
            p.trailing_comma(i >= self.args.len() - 1);
        }
        p.word(")");
    }
}
