use crate::fmt;
use pg_query::protobuf::OnCommitAction;

impl fmt::Print for OnCommitAction {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        match self {
            OnCommitAction::Undefined => Ok(()),
            OnCommitAction::OncommitNoop => Ok(()),
            OnCommitAction::OncommitPreserveRows => Ok(p.keyword(" on commit preserve rows")),
            OnCommitAction::OncommitDeleteRows => Ok(p.keyword(" on commit delete rows")),
            OnCommitAction::OncommitDrop => Ok(p.keyword(" on commit drop")),
        }
    }
}
