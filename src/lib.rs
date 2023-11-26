#![allow(unused)]

mod algorithm;
mod convenience;
mod ring;
mod stmt;
mod tree;

use crate::algorithm::Printer;
use pg_query::protobuf::ParseResult;

const MARGIN: isize = 89;
const INDENT: isize = 4;
const MIN_SPACE: isize = 60;

pub fn unparse(tree: &ParseResult) -> String {
    let mut p = Printer::new();
    p.tree(tree);
    p.eof()
}
