use crate::fmt;
use crate::fmt::gram::print_non_reserved_word_or_scont;
use crate::fmt::gram::str_val;
use pg_query::protobuf::CreateExtensionStmt;
use pg_query::NodeEnum;

impl fmt::Print for CreateExtensionStmt {
    fn print(&self, p: &mut fmt::Printer) -> fmt::Result {
        p.keyword("create extension ");

        if self.if_not_exists {
            p.keyword("if not exists ");
        }

        p.ident(self.extname.clone());
        p.nbsp();

        for option in &self.options {
            let def_elem = option
                .node
                .as_ref()
                .and_then(|option| match option {
                    NodeEnum::DefElem(def_elem) => Some(def_elem),
                    _ => None,
                })
                .unwrap();

            match def_elem.defname.as_ref() {
                "schema" => {
                    p.keyword("schema ");
                    p.ident(str_val(&def_elem.arg.clone().unwrap()).unwrap());
                }
                "new_version" => {
                    p.keyword("version ");
                    print_non_reserved_word_or_scont(
                        p,
                        str_val(&def_elem.arg.clone().unwrap()).unwrap(),
                    )?;
                }
                "cascade" => {
                    p.keyword("cascade");
                }
                _ => unreachable!("{}", def_elem.defname),
            }

            p.space()
        }

        Ok(())
    }
}
