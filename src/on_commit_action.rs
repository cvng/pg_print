use crate::fmt;
use pg_query::protobuf::OnCommitAction;

impl fmt::Print for OnCommitAction {
    fn print(&self, p: &mut fmt::Printer) {
        match self {
            OnCommitAction::Undefined => {}
            OnCommitAction::OncommitNoop => {}
            OnCommitAction::OncommitPreserveRows => p.word(" on commit preserve rows"),
            OnCommitAction::OncommitDeleteRows => p.word(" on commit delete rows"),
            OnCommitAction::OncommitDrop => p.word(" on commit drop"),
        }
    }
}
