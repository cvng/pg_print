#![allow(dead_code)]

#[derive(Default)]
pub enum DeparseNodeContext {
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
}

#[derive(Default)]
pub struct Context {
    pub context: DeparseNodeContext,
    pub is_foreign_table: bool,
}
