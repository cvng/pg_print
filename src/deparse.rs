use crate::error::Result;
use pg_query::protobuf;

/// Converts a parsed tree back into a string.
pub fn deparse(protobuf: &protobuf::ParseResult) -> Result<String> {
    pg_query::deparse(protobuf)
}
