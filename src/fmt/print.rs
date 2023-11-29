use std::option;

pub trait Print {
    fn print(&self, p: &mut super::Printer) -> Option;
}

pub type Option = option::Option<()>;
