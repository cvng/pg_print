use crate::fmt;
use crate::fmt::Print;
use crate::interval_fields::IntervalFields;
use crate::interval_fields::INTERVAL_FULL_PRECISION;
use crate::utils::a_const_int_val;
use crate::utils::int_val;
use crate::utils::print_any_name;
use crate::utils::str_val;
use pg_query::protobuf::Integer;
use pg_query::protobuf::TypeName;
use pg_query::Node;
use pg_query::NodeEnum;

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

fn print_interval_typmods(p: &mut fmt::Printer, node: &TypeName) {
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

    IntervalFields::try_from(interval_fields).unwrap().print(p);

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
            p.word(format!(" ({})", precision));
        }
    }
}

fn print_signed_iconst(p: &mut fmt::Printer, node: &Node) {
    p.word(format!("{}", int_val(node).unwrap()));
}
