use crate::fmt;
use crate::fmt::a_const_int_val;
use crate::fmt::gram;
use crate::fmt::int_val;
use crate::fmt::str_val;
use crate::fmt::Print;
use crate::interval_fields::IntervalFields;
use crate::interval_fields::INTERVAL_FULL_PRECISION;
use crate::name::Name;
use pg_query::protobuf::Integer;
use pg_query::protobuf::TypeName;
use pg_query::Node;
use pg_query::NodeEnum;

impl fmt::Print for TypeName {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        let mut skip_typmods = false;

        if self.setof {
            p.keyword("setof ");
        }

        if self.names.len() == 2 && str_val(self.names.first().unwrap()).unwrap() == "pg_catalog" {
            let name = str_val(self.names.last().unwrap()).unwrap();

            match name.clone().into() {
                Name::Bpchar => p.word("char"),
                Name::Varchar => p.word("varchar"),
                Name::Numeric => p.word("numeric"),
                Name::Bool => p.word("boolean"),
                Name::Int2 => p.word("smallint"),
                Name::Int4 => p.word("int"),
                Name::Int8 => p.word("bigint"),
                Name::Real => p.word("real"),
                Name::Float8 => p.word("double precision"),
                Name::Time => p.word("time"),
                Name::Timetz => {
                    skip_typmods = true;
                    p.word("time ");

                    if !self.typmods.is_empty() {
                        p.word("(");
                        for (i, typmod) in self.typmods.iter().enumerate() {
                            gram::signed_iconst(p, typmod);
                            p.comma(i >= self.typmods.len() - 1);
                        }
                        p.word(") ");
                    }

                    p.word("with time zone")
                }
                Name::Timestamp => p.word("timestamp"),
                Name::Timestamptz => {
                    skip_typmods = true;
                    p.word("timestamp ");

                    if !self.typmods.is_empty() {
                        p.word("(");
                        for (i, typmod) in self.typmods.iter().enumerate() {
                            gram::signed_iconst(p, typmod);
                            p.comma(i >= self.typmods.len() - 1);
                        }
                        p.word(") ");
                    }
                    p.word("with time zone")
                }
                Name::Interval => {
                    p.word("interval");

                    if !self.typmods.is_empty() {
                        skip_typmods = true;
                        print_interval_typmods(p, self)?;
                    }
                }
                Name::Undefined => {
                    p.word("pg_catalog.");
                    p.word(name)
                }
            };
        } else {
            gram::any_name(p, &self.names)?;
        }

        if !self.typmods.is_empty() && !skip_typmods {
            p.word("(");
            for (i, typmod) in self.typmods.iter().enumerate() {
                typmod.print(p)?;
                p.comma(i >= self.typmods.len() - 1);
            }
            p.word(")");
        }

        Ok(())
    }
}

fn print_interval_typmods(p: &mut fmt::Printer, node: &TypeName) -> fmt::Result {
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

    IntervalFields::from(interval_fields).print(p)?;

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

    Ok(())
}
