//! https://github.com/lelit/pglast/blob/v5/tests/test_printers_prettification.py

use pg_deparser::stream::IndentedStream;

#[test]
fn test_prettification() {
    for src in glob::glob("**/*.sql").unwrap().filter_map(Result::ok) {
        let mut lineno = 1;

        for case in std::fs::read_to_string(&src)
            .unwrap()
            .split("\n\n")
            .map(|case| case.trim())
        {
            lineno += case.matches('\n').count() + 2;

            let parts = case.split("\n=\n").collect::<Vec<_>>();
            let original = parts[0].trim();
            let parts = parts[1].split("\n:\n").collect::<Vec<_>>();

            let mut expected = parts[0].trim().replace("\\n\\\n", "\n").replace("\\s", " ");
            if expected.ends_with('\\') {
                expected = expected[0..expected.len() - 1].to_owned() + "\n"
            }

            let prettified = IndentedStream::new().call(original);

            assert_eq!(
                expected,
                prettified,
                "{}:{}:{} != {}",
                src.display(),
                lineno,
                expected,
                prettified
            );
        }
    }
}
