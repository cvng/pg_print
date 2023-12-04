use crate::fmt::Printer;
use pg_query::protobuf::OnCommitAction;

impl Printer {
    pub fn on_commit_action(&mut self, n: &OnCommitAction) {
        match n {
            OnCommitAction::Undefined => {}
            OnCommitAction::OncommitNoop => {}
            OnCommitAction::OncommitPreserveRows => self.word(" on commit preserve rows"),
            OnCommitAction::OncommitDeleteRows => self.word(" on commit delete rows"),
            OnCommitAction::OncommitDrop => self.word(" on commit drop"),
        }
    }
}
