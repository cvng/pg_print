use crate::fmt::Printer;
use crate::gram::str_val;
use pg_query::protobuf::CreateExtensionStmt;
use pg_query::NodeEnum;

impl Printer {
    pub fn create_extension_stmt(&mut self, n: &CreateExtensionStmt) {
        self.word("create extension ");
        self.optional_word("if not exists ", n.if_not_exists);
        self.ident(n.extname.clone());
        self.nbsp();

        for option in &n.options {
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
                    self.word("schema ");
                    self.ident(str_val(&def_elem.arg.clone().unwrap()).unwrap());
                }
                "new_version" => {
                    self.word("version ");
                    self.non_reserved_word_or_scont(
                        str_val(&def_elem.arg.clone().unwrap()).unwrap(),
                    );
                }
                "cascade" => {
                    self.word("cascade ");
                }
                _ => {}
            }
            self.space();
        }
    }
}
