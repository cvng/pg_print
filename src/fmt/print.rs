#![allow(dead_code)]

use std::result;

pub type Result = result::Result<(), Error>;

pub struct Error;

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
    #[inline]
    fn print(&self, p: &mut super::Printer) -> Result {
        self.print_in_context(p, &Context::default())
    }

    #[inline]
    fn print_in_context(&self, _p: &mut super::Printer, _ctx: &Context) -> Result {
        Ok(())
    }
}
