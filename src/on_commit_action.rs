use crate::fmt;
use pg_query::protobuf::OnCommitAction;

impl fmt::Print for OnCommitAction {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        match self {
            OnCommitAction::Undefined => {}
            OnCommitAction::OncommitNoop => {}
            OnCommitAction::OncommitPreserveRows => p.keyword(" on commit preserve rows"),
            OnCommitAction::OncommitDeleteRows => p.keyword(" on commit delete rows"),
            OnCommitAction::OncommitDrop => p.keyword(" on commit drop"),
        };

        Some(())
    }
}
