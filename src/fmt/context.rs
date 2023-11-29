#![allow(dead_code)]

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
