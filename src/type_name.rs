use crate::fmt::Printer;
use crate::gram::a_const_int_val;
use crate::gram::int_val;
use crate::gram::str_val;
use crate::interval_fields::IntervalFields;
use crate::interval_fields::INTERVAL_FULL_PRECISION;
use crate::name::Name;
use pg_query::protobuf::Integer;
use pg_query::protobuf::TypeName;
use pg_query::Node;
use pg_query::NodeEnum;

impl Printer {
    pub fn type_name(&mut self, n: &TypeName) {
        let mut skip_typmods = false;

        if n.setof {
            self.word("setof ");
        }

        if n.names.len() == 2 && str_val(n.names.first().unwrap()).unwrap() == "pg_catalog" {
            let name = str_val(n.names.last().unwrap()).unwrap();

            match name.clone().into() {
                Name::Bpchar => self.word("char"),
                Name::Varchar => self.word("varchar"),
                Name::Numeric => self.word("numeric"),
                Name::Bool => self.word("boolean"),
                Name::Int2 => self.word("smallint"),
                Name::Int4 => self.word("int"),
                Name::Int8 => self.word("bigint"),
                Name::Real => self.word("real"),
                Name::Float8 => self.word("double precision"),
                Name::Time => self.word("time"),
                Name::Timetz => {
                    skip_typmods = true;
                    self.word("time ");

                    if !n.typmods.is_empty() {
                        self.word("(");
                        for (i, typmod) in n.typmods.iter().enumerate() {
                            self.signed_iconst(typmod);
                            self.trailing_comma(i >= n.typmods.len() - 1);
                        }
                        self.word(") ");
                    }

                    self.word("with time zone")
                }
                Name::Timestamp => self.word("timestamp"),
                Name::Timestamptz => {
                    skip_typmods = true;
                    self.word("timestamp ");

                    if !n.typmods.is_empty() {
                        self.word("(");
                        for (i, typmod) in n.typmods.iter().enumerate() {
                            self.signed_iconst(typmod);
                            self.trailing_comma(i >= n.typmods.len() - 1);
                        }
                        self.word(") ");
                    }
                    self.word("with time zone")
                }
                Name::Interval => {
                    self.word("interval");

                    if !n.typmods.is_empty() {
                        skip_typmods = true;
                        self.interval_typmods(n);
                    }
                }
                Name::Undefined => {
                    self.word("pg_catalog.");
                    self.word(name)
                }
            };
        } else {
            self.any_name(&n.names);
        }

        if !n.typmods.is_empty() && !skip_typmods {
            self.word("(");
            for (i, typmod) in n.typmods.iter().enumerate() {
                self.node(typmod);
                self.trailing_comma(i >= n.typmods.len() - 1);
            }
            self.word(")");
        }
    }

    pub fn interval_typmods(&mut self, node: &TypeName) {
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

        self.interval_fields(&IntervalFields::from(interval_fields));

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
                self.word(format!(" ({})", precision));
            }
        }
    }
}
