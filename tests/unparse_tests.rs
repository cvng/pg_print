// Adapted from https://github.com/lelit/pglast/blob/v5/tests/test_printers_prettification.py.

use pg_deparser::unparse;
use pg_query::parse;

const PASSES: [&str; 2] = [
    // "tests/sql/ddl/alter_default_privileges.sql",
    // "tests/sql/ddl/alter_subscription.sql",
    // "tests/sql/ddl/alter_table.sql",
    // "tests/sql/ddl/alter_text_search.sql",
    // "tests/sql/ddl/comment.sql",
    // "tests/sql/ddl/create_aggregate.sql",
    // "tests/sql/ddl/create_database.sql",
    // "tests/sql/ddl/create_domain.sql",
    // "tests/sql/ddl/create_event_trigger.sql",
    // "tests/sql/ddl/create_extension.sql",
    // "tests/sql/ddl/create_foreign_data_wrapper.sql",
    // "tests/sql/ddl/create_foreign_table.sql",
    // "tests/sql/ddl/create_function.sql",
    // "tests/sql/ddl/create_index.sql",
    // "tests/sql/ddl/create_language.sql",
    // "tests/sql/ddl/create_rule.sql",
    // "tests/sql/ddl/create_schema.sql",
    // "tests/sql/ddl/create_sequence.sql",
    "tests/sql/ddl/create_table.sql",
    // "tests/sql/ddl/create_transform.sql",
    // "tests/sql/ddl/create_trigger.sql",
    "tests/sql/ddl/create_type.sql",
    // "tests/sql/ddl/create_view.sql",
    // "tests/sql/ddl/grant.sql",
    // "tests/sql/ddl/issue110.sql",
    // "tests/sql/ddl/notify.sql",
    // "tests/sql/ddl/show.sql",
];

#[test]
fn test_unparse_statements() {
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

            let prettified = unparse(&parse(original).unwrap().protobuf)
                .unwrap()
                .trim_end()
                .to_string();

            assert_eq!(expected, prettified, "{}:{}:", src.display(), lineno);
        }
    }
}
