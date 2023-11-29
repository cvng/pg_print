use crate::create_stmt::node_any_name;
use crate::create_stmt::node_interval_typmods;
use crate::create_stmt::node_signed_iconst;
use crate::fmt;
use crate::utils::str_val;
use pg_query::protobuf::TypeName;
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
                            node_signed_iconst(p, typmod);
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
                            node_signed_iconst(p, typmod);
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
                        node_interval_typmods(p, self);
                        skip_typmods = true;
                    }
                }
                _ => {
                    p.word("pg_catalog.");
                    p.word(name);
                }
            }
        } else {
            node_any_name(p, &self.names);
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
