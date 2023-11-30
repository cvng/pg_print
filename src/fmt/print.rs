#![allow(dead_code)]

use std::result;

pub type Result = result::Result<(), Error>;

pub struct Error;

/// Deparse node context: parent / field / identifier / constant.
#[derive(Default)]
pub enum Context {
    #[default]
    None,
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
    ForeignRelation,
    Identifier,
    Constant,
}

pub trait Print {
    fn print(&self, p: &mut super::Printer) -> Result {
        self.print_in_context(p, &Context::default())
    }

    fn print_in_context(&self, _p: &mut super::Printer, _ctx: &Context) -> Result {
        Ok(())
    }
}
