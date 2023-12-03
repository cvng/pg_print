// Adapted from https://github.com/lelit/pglast/blob/v5/tests/test_printers_prettification.py.

use insta::assert_snapshot;
use pg_query::parse;
use rstest::rstest;
use std::env;
use std::fs;
use std::path::PathBuf;

macro_rules! set_snapshot_suffix {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_suffix(format!($($expr,)*));
        let _guard = settings.bind_to_scope();
    }
}

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

        set_snapshot_suffix!("{}", lineno);

        let deparsed = pg_query::deparse(&parse(case).unwrap().protobuf)
            .unwrap()
            .trim_end()
            .to_string();

        let unparsed = pg_deparser::unparse(&parse(case).unwrap().protobuf)
            .unwrap()
            .trim_end()
            .to_string();

        let reparsed = pg_query::deparse(&parse(&unparsed).unwrap().protobuf)
            .unwrap()
            .trim_end()
            .to_string();

        assert_eq!(deparsed, reparsed);

        assert_snapshot!(
            path.to_str().unwrap().replace(['/', '.'], "_"),
            unparsed,
            &format!("{}:{}", path.display(), lineno)
        );
    }
}
