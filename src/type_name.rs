use crate::fmt;
use crate::utils::a_const_int_val;
use crate::utils::int_val;
use crate::utils::str_val;
use pg_query::protobuf::Integer;
use pg_query::protobuf::TypeName;
use pg_query::Node;
use pg_query::NodeEnum;

const MONTH: i32 = 1;
const YEAR: i32 = 2;
const DAY: i32 = 3;
const HOUR: i32 = 10;
const MINUTE: i32 = 11;
const SECOND: i32 = 12;

const INTERVAL_FULL_RANGE: i32 = 0x7FFF;
const INTERVAL_FULL_PRECISION: i32 = 0xFFFF;

impl fmt::Print for TypeName {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Option {
        let mut skip_typmods = false;

        if self.setof {
            p.keyword("setof ");
        }

        if self.names.len() == 2 && str_val(self.names.first().unwrap()).unwrap() == "pg_catalog" {
            let name = str_val(self.names.last().unwrap()).unwrap();

            match name.as_str() {
                "bpchar" => p.word("char"),
                "varchar" => p.word("varchar"),
                "numeric" => p.word("numeric"),
                "bool" => p.word("boolean"),
                "int2" => p.word("smallint"),
                "int4" => p.word("int"),
                "int8" => p.word("bigint"),
                "real" | "float4" => p.word("real"),
                "float8" => p.word("double precision"),
                "time" => p.word("time"),
                "timetz" => {
                    p.word("time ");
                    if !self.typmods.is_empty() {
                        p.word("(");
                        for (i, typmod) in self.typmods.iter().enumerate() {
                            print_signed_iconst(p, typmod);
                            p.comma(i >= self.typmods.len() - 1);
                        }
                        p.word(") ");
                    }
                    p.word("with time zone");
                    skip_typmods = true;
                }
                "timestamp" => p.word("timestamp"),
                "timestamptz" => {
                    p.word("timestamp ");
                    if !self.typmods.is_empty() {
                        p.word("(");
                        for (i, typmod) in self.typmods.iter().enumerate() {
                            print_signed_iconst(p, typmod);
                            p.comma(i >= self.typmods.len() - 1);
                        }
                        p.word(") ");
                    }
                    p.word("with time zone");
                    skip_typmods = true;
                }
                "interval" => {
                    p.word("interval");

                    if !self.typmods.is_empty() {
                        print_interval_typmods(p, self);
                        skip_typmods = true;
                    }
                }
                _ => {
                    p.word("pg_catalog.");
                    p.word(name);
                }
            }
        } else {
            print_any_name(p, &self.names);
        }

        if !self.typmods.is_empty() && !skip_typmods {
            p.word("(");
            for (i, typmod) in self.typmods.iter().enumerate() {
                match typmod.node.as_ref().unwrap() {
                    NodeEnum::AConst(node) => node.print(p)?,
                    NodeEnum::ParamRef(node) => node.print(p)?,
                    NodeEnum::ColumnRef(node) => node.print(p)?,
                    _ => unreachable!(),
                }
                p.comma(i >= self.typmods.len() - 1);
            }
            p.word(")");
        }

        Some(())
    }
}

// See https://github.com/pganalyze/libpg_query/blob/15-latest/src/postgres_deparse.c#L3774.
fn print_interval_typmods(str: &mut fmt::Printer, node: &TypeName) {
    let interval_fields = node
        .typmods
        .first()
        .and_then(a_const_int_val)
        .map(|ival| Some(NodeEnum::Integer(Integer { ival })))
        .map(|node| Node { node })
        .as_ref()
        .map(int_val)
        .unwrap()
        .unwrap();

    match interval_fields {
        x if x == 1 << YEAR => str.word(" year"),
        x if x == 1 << MONTH => str.word(" month"),
        x if x == 1 << DAY => str.word(" day"),
        x if x == 1 << HOUR => str.word(" hour"),
        x if x == 1 << MINUTE => str.word(" minute"),
        x if x == 1 << SECOND => str.word(" second"),
        x if x == 1 << YEAR | 1 << MONTH => str.word(" year to month"),
        x if x == 1 << DAY | 1 << HOUR => str.word(" day to hour"),
        x if x == 1 << DAY | 1 << HOUR | 1 << MINUTE => str.word(" day to minute"),
        x if x == 1 << DAY | 1 << HOUR | 1 << MINUTE | 1 << SECOND => str.word(" day to second"),
        x if x == 1 << HOUR | 1 << MINUTE => str.word(" hour to minute"),
        x if x == 1 << HOUR | 1 << MINUTE | 1 << SECOND => str.word(" hour to second"),
        x if x == 1 << MINUTE | 1 << SECOND => str.word(" minute to second"),
        INTERVAL_FULL_RANGE => {}
        _ => unreachable!(),
    };

    if node.typmods.len() == 2 {
        let precision = node
            .typmods
            .last()
            .and_then(a_const_int_val)
            .map(|ival| Some(NodeEnum::Integer(Integer { ival })))
            .map(|node| Node { node })
            .as_ref()
            .map(int_val)
            .unwrap()
            .unwrap();

        if precision != INTERVAL_FULL_PRECISION {
            str.word(format!(" ({})", precision));
        }
    }
}

pub fn print_any_name(str: &mut fmt::Printer, list: &[Node]) -> fmt::Option {
    for (i, part) in list.iter().enumerate() {
        if i > 0 {
            str.word(".");
        }
        str.ident(str_val(part).unwrap());
    }

    Some(())
}

fn print_signed_iconst(str: &mut fmt::Printer, node: &Node) {
    str.word(format!("{}", int_val(node).unwrap()));
}
