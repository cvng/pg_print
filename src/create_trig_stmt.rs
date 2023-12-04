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

        self.word("create ");

        if self.replace {
            self.word("or replace ");
        }

        if self.isconstraint {
            self.word("constraint ");
        }

        self.word("trigger ");
        self.ident(self.trigname.clone());
        self.nbsp();

        match self.timing as usize {
            TRIGGER_TYPE_BEFORE => self.word("before "),
            TRIGGER_TYPE_AFTER => self.word("after "),
            TRIGGER_TYPE_INSTEAD => self.word("instead of "),
            _ => {}
        }

        if self.events as usize & TRIGGER_TYPE_INSERT != 0 {
            self.word("insert ");
            skip_events_or = false;
        }

        if self.events as usize & TRIGGER_TYPE_DELETE != 0 {
            if !skip_events_or {
                self.word("or ");
            }
            self.word("delete ");
            skip_events_or = false;
        }

        if self.events as usize & TRIGGER_TYPE_UPDATE != 0 {
            if !skip_events_or {
                self.word("or ");
            }
            self.word("update ");

            if !self.columns.is_empty() {
                self.word("of ");
                self.column_list(&self.columns);
                self.nbsp();
            }
            skip_events_or = false;
        }

        if self.events as usize & TRIGGER_TYPE_TRUNCATE != 0 {
            if !skip_events_or {
                self.word("or ");
            }
            self.word("truncate ");
        }

        self.word("on ");
        self.relation.as_ref().unwrap().print(p);
        self.nbsp();

        if !self.transition_rels.is_empty() {
            self.word("referencing ");
            for transition_rel in &self.transition_rels {
                self.node(transition_rel);
                self.nbsp();
            }
        }

        if let Some(constrrel) = &self.constrrel {
            self.word("from ");
            constrrel.print(p);
            self.nbsp();
        }

        if self.deferrable {
            self.word("deferrable ");
        }

        if self.initdeferred {
            self.word("initially deferred ");
        }

        if self.row {
            self.word("for each row ");
        }

        if let Some(when_clause) = &self.when_clause {
            self.word("when (");
            self.node(when_clause);
            self.word(") ");
        }

        self.word("execute function ");
        self.func_name(&self.funcname);

        self.word("(");
        for (i, arg) in self.args.iter().enumerate() {
            string_literal(p, &str_val(arg).unwrap());
            self.trailing_comma(i >= self.args.len() - 1);
        }
        self.word(")");
    }
}
