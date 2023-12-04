use crate::fmt::Printer;
use pg_query::protobuf::CreateTrigStmt;
use crate::gram::string_literal;
use crate::gram::str_val;

const TRIGGER_TYPE_BEFORE: usize = 1 << 1;
const TRIGGER_TYPE_INSERT: usize = 1 << 2;
const TRIGGER_TYPE_DELETE: usize = 1 << 3;
const TRIGGER_TYPE_UPDATE: usize = 1 << 4;
const TRIGGER_TYPE_TRUNCATE: usize = 1 << 5;
const TRIGGER_TYPE_INSTEAD: usize = 1 << 6;
const TRIGGER_TYPE_AFTER: usize = 0;

impl Printer {
    pub fn create_trig_stmt(&mut self, n: &CreateTrigStmt) {
        let mut skip_events_or = true;

        self.word("create ");

        if n.replace {
            self.word("or replace ");
        }

        if n.isconstraint {
            self.word("constraint ");
        }

        self.word("trigger ");
        self.ident(n.trigname.clone());
        self.nbsp();

        match n.timing as usize {
            TRIGGER_TYPE_BEFORE => self.word("before "),
            TRIGGER_TYPE_AFTER => self.word("after "),
            TRIGGER_TYPE_INSTEAD => self.word("instead of "),
            _ => {}
        }

        if n.events as usize & TRIGGER_TYPE_INSERT != 0 {
            self.word("insert ");
            skip_events_or = false;
        }

        if n.events as usize & TRIGGER_TYPE_DELETE != 0 {
            if !skip_events_or {
                self.word("or ");
            }
            self.word("delete ");
            skip_events_or = false;
        }

        if n.events as usize & TRIGGER_TYPE_UPDATE != 0 {
            if !skip_events_or {
                self.word("or ");
            }
            self.word("update ");

            if !n.columns.is_empty() {
                self.word("of ");
                self.column_list(&n.columns);
                self.nbsp();
            }
            skip_events_or = false;
        }

        if n.events as usize & TRIGGER_TYPE_TRUNCATE != 0 {
            if !skip_events_or {
                self.word("or ");
            }
            self.word("truncate ");
        }

        self.word("on ");
        self.range_var(n.relation.as_ref().unwrap());
        self.nbsp();

        if !n.transition_rels.is_empty() {
            self.word("referencing ");
            for transition_rel in &n.transition_rels {
                self.node(transition_rel);
                self.nbsp();
            }
        }

        if let Some(constrrel) = &n.constrrel {
            self.word("from ");
            self.range_var(constrrel);
            self.nbsp();
        }

        if n.deferrable {
            self.word("deferrable ");
        }

        if n.initdeferred {
            self.word("initially deferred ");
        }

        if n.row {
            self.word("for each row ");
        }

        if let Some(when_clause) = &n.when_clause {
            self.word("when (");
            self.node(when_clause);
            self.word(") ");
        }

        self.word("execute function ");
        self.func_name(&n.funcname);

        self.word("(");
        for (i, arg) in n.args.iter().enumerate() {
            string_literal(self, &str_val(arg).unwrap());
            self.trailing_comma(i >= n.args.len() - 1);
        }
        self.word(")");
    }
}
