use insta::assert_snapshot;
use pg_query::parse;
use std::env;
use std::fs;
use std::path::PathBuf;

#[test]
fn unparse() {
    let path = PathBuf::from("tests/unparse.sql");

    for (mut lineno, case) in fs::read_to_string(&path).unwrap().lines().enumerate() {
        lineno += 1;

        if case.starts_with("--") {
            continue;
        }

        let deparsed = pg_query::deparse(&parse(case).unwrap().protobuf)
            .unwrap()
            .to_string();

        let unparsed = pg_print::unparse(&parse(case).unwrap().protobuf)
            .unwrap()
            .to_string();

        let reparsed = pg_query::deparse(&parse(&unparsed).unwrap().protobuf)
            .unwrap()
            .to_string();

        let fingerprint = pg_query::fingerprint(case).unwrap().hex.to_string();

        let description = format!("{}:{}", path.display(), lineno);

        assert_eq!(deparsed, reparsed, "{}", &description);

        assert_snapshot!(fingerprint, unparsed, &description);
    }
}
