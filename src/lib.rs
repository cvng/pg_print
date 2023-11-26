#![allow(unused)]

mod algorithm;
mod convenience;
mod node;
mod ring;
mod tree;

use crate::algorithm::Printer;
use pg_query::protobuf;

const MARGIN: isize = 89;
const INDENT: isize = 4;
const MIN_SPACE: isize = 60;

pub fn unparse(protobuf: &protobuf::ParseResult) -> pg_query::Result<String> {
    let mut p = Printer::new();
    p.tree(protobuf);
    Ok(p.eof())
}
