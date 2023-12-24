use insta::assert_snapshot;
use std::fs;
use std::path::Path;

#[test]
fn unparse() {
    let path = Path::new("tests/unparse_tests.sql");

    for (mut line, statement) in fs::read_to_string(path).unwrap().lines().enumerate() {
        line += 1;

        if statement.starts_with("--") {
            continue;
        }

        let unparsed = pg_print::unparse(&pg_print::parse(statement).unwrap());
        let deparsed = pg_query::deparse(&pg_query::parse(statement).unwrap().protobuf).unwrap();
        let reparsed = pg_query::deparse(&pg_query::parse(&unparsed).unwrap().protobuf).unwrap();

        let fingerprint = pg_query::fingerprint(statement).unwrap().hex;
        let description = format!("{}:{}", path.display(), line);

        assert_eq!(deparsed, reparsed, "{}", &description);
        assert_snapshot!(fingerprint, unparsed, &description);
    }
}
