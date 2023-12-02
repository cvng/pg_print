// Adapted from https://github.com/lelit/pglast/blob/v5/tests/test_printers_prettification.py.

use insta::assert_snapshot;
use insta::with_settings;
use pg_deparser::unparse;
use pg_query::parse;

const PASSES: [&str; 10] = [
    // "tests/sql/ddl/alter_default_privileges.sql",
    // "tests/sql/ddl/alter_subscription.sql",
    // "tests/sql/ddl/alter_table.sql",
    // "tests/sql/ddl/alter_text_search.sql",
    // "tests/sql/ddl/comment.sql",
    // "tests/sql/ddl/create_aggregate.sql",
    // "tests/sql/ddl/create_database.sql",
    "tests/sql/ddl/create_domain.sql",
    // "tests/sql/ddl/create_event_trigger.sql",
    "tests/sql/ddl/create_extension.sql",
    // "tests/sql/ddl/create_foreign_data_wrapper.sql",
    "tests/sql/ddl/create_foreign_table.sql",
    "tests/sql/ddl/create_function.sql",
    // "tests/sql/ddl/create_index.sql",
    // "tests/sql/ddl/create_language.sql",
    // "tests/sql/ddl/create_rule.sql",
    "tests/sql/ddl/create_schema.sql",
    // "tests/sql/ddl/create_sequence.sql",
    "tests/sql/ddl/create_table.sql",
    // "tests/sql/ddl/create_transform.sql",
    "tests/sql/ddl/create_trigger.sql",
    "tests/sql/ddl/create_type.sql",
    "tests/sql/ddl/create_view.sql",
    "tests/sql/ddl/grant.sql",
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
        for (mut lineno, case) in std::fs::read_to_string(&src)
            .unwrap()
            .split('\n')
            .filter(|line| !line.is_empty())
            .enumerate()
        {
            lineno += 1;

            let prettified = unparse(&parse(case).unwrap().protobuf)
                .unwrap()
                .trim_end()
                .to_string();

            with_settings!({
                description => format!("Statement: {}:{}", src.display(), lineno),
            }, {
                assert_snapshot!(
                    format!("{}_{}", src.file_stem().unwrap().to_string_lossy(), lineno),
                    prettified
                );
            });
        }
    }
}
