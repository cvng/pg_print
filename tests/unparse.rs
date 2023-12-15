use insta::assert_snapshot;
use pg_query::parse;
use std::fs;
use std::path::Path;

#[cfg(feature = "unstable")]
fn parse_source(text: &str) -> parser::Parse {
    parser::parse_source(text)
}

#[cfg(not(feature = "unstable"))]
fn parse_source(text: &str) -> pg_query::protobuf::ParseResult {
    pg_query::parse(text).unwrap().protobuf
}

#[test]
fn unparse() {
    let path = Path::new("tests/unparse_tests.sql");

    for (mut line, case) in fs::read_to_string(path).unwrap().lines().enumerate() {
        line += 1;

        if case.starts_with("--") {
            continue;
        }

        let unparsed = pg_print::unparse(&parse_source(case));
        let deparsed = pg_query::deparse(&parse(case).unwrap().protobuf).unwrap();
        let reparsed = pg_query::deparse(&parse(&unparsed).unwrap().protobuf).unwrap();

        let fingerprint = pg_query::fingerprint(case).unwrap().hex;
        let description = format!("{}:{}", path.display(), line);

        assert_eq!(deparsed, reparsed, "{}", &description);
        assert_snapshot!(fingerprint, unparsed, &description);
    }
}
