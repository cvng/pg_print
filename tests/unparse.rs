use insta::assert_snapshot;
use pg_query::parse;
use std::fs;
use std::path::PathBuf;

#[test]
fn unparse() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/unparse.sql");

    for (mut lineno, case) in fs::read_to_string(&path)
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("--"))
        .enumerate()
    {
        lineno += 1;

        let deparsed = pg_query::deparse(&parse(case).unwrap().protobuf)
            .unwrap()
            .trim_end()
            .to_string();

        let unparsed = pg_print::unparse(&parse(case).unwrap().protobuf)
            .unwrap()
            .trim_end()
            .to_string();

        let reparsed = pg_query::deparse(&parse(&unparsed).unwrap().protobuf)
            .unwrap()
            .trim_end()
            .to_string();

        assert_eq!(deparsed, reparsed);

        assert_snapshot!(
            lineno.to_string(),
            unparsed,
            &format!("{}:{}", path.display(), lineno)
        );
    }
}
