// Adapted from https://github.com/lelit/pglast/blob/v5/tests/test_printers_prettification.py.

use insta::assert_snapshot;
use pg_query::parse;
use rstest::rstest;
use std::env;
use std::fs;
use std::path::PathBuf;

#[rstest]
fn unparse(
    #[files("tests/**/*.sql")]
    #[exclude("exclude")]
    path: PathBuf,
) {
    let path = path.strip_prefix(env::current_dir().unwrap()).unwrap();

    for (mut lineno, case) in fs::read_to_string(path)
        .unwrap()
        .split('\n')
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
            format!(
                "{}_{}",
                path.to_str().unwrap().replace(['/', '.'], "_"),
                lineno
            ),
            unparsed,
            &format!("{}:{}", path.display(), lineno)
        );
    }
}
