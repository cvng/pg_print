// https://github.com/lelit/pglast/blob/v5/tests/test_printers_prettification.py

use pg_deparser::stream::IndentedStream;

const PASSES: [&str; 1] = [
    // "tests/test_printers_prettification/ddl/alter_default_privileges.sql",
    // "tests/test_printers_prettification/ddl/alter_subscription.sql",
    // "tests/test_printers_prettification/ddl/alter_table.sql",
    // "tests/test_printers_prettification/ddl/alter_text_search.sql",
    // "tests/test_printers_prettification/ddl/comment.sql",
    // "tests/test_printers_prettification/ddl/create_aggregate.sql",
    // "tests/test_printers_prettification/ddl/create_database.sql",
    // "tests/test_printers_prettification/ddl/create_domain.sql",
    // "tests/test_printers_prettification/ddl/create_event_trigger.sql",
    // "tests/test_printers_prettification/ddl/create_extension.sql",
    // "tests/test_printers_prettification/ddl/create_foreign_data_wrapper.sql",
    // "tests/test_printers_prettification/ddl/create_foreign_table.sql",
    // "tests/test_printers_prettification/ddl/create_function.sql",
    // "tests/test_printers_prettification/ddl/create_index.sql",
    // "tests/test_printers_prettification/ddl/create_language.sql",
    // "tests/test_printers_prettification/ddl/create_rule.sql",
    // "tests/test_printers_prettification/ddl/create_schema.sql",
    // "tests/test_printers_prettification/ddl/create_sequence.sql",
    // "tests/test_printers_prettification/ddl/create_table.sql",
    // "tests/test_printers_prettification/ddl/create_transform.sql",
    // "tests/test_printers_prettification/ddl/create_trigger.sql",
    "tests/test_printers_prettification/ddl/create_type.sql",
    // "tests/test_printers_prettification/ddl/create_view.sql",
    // "tests/test_printers_prettification/ddl/grant.sql",
    // "tests/test_printers_prettification/ddl/issue110.sql",
    // "tests/test_printers_prettification/ddl/notify.sql",
    // "tests/test_printers_prettification/ddl/show.sql",
];

#[test]
fn test_prettification() {
    for src in glob::glob("**/*.sql")
        .unwrap()
        .filter_map(Result::ok)
        .filter(|path| PASSES.contains(&path.to_str().unwrap()))
    {
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

            let prettified = IndentedStream::default().call(original);

            assert_eq!(expected, prettified, "{}:{}:", src.display(), lineno);
        }
    }
}
