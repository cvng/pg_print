#![allow(dead_code)]

use std::option;

pub type Option = option::Option<()>;

#[derive(Default)]
pub enum Context {
    #[default]
    None,
    // Parent node type (and sometimes field).
    InsertRelation,
    InsertOnConflict,
    Update,
    Returning,
    AExpr,
    Xmlattributes,
    Xmlnamespaces,
    CreateType,
    AlterType,
    SetStatement,
    // Identifier vs constant context.
    Identifier,
    Constant,
    // Other.
    ForeignTable,
}

pub trait Print {
    fn print(&self, p: &mut super::Printer) -> Option {
        self.print_in_context(p, &Context::default())
    }

    fn print_in_context(&self, _p: &mut super::Printer, _ctx: &Context) -> Option {
        Some(())
    }
}