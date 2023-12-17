use crate::fmt::Printer;
use pg_query::protobuf::AccessPriv;
use pg_query::protobuf::GrantTargetType;
use pg_query::protobuf::ObjectType;
use pg_query::Node;

impl Printer {
    pub fn access_priv(&mut self, n: &AccessPriv) {
        if !n.priv_name.is_empty() {
            match n.priv_name.as_ref() {
                "select" => self.word("select"),
                "references" => self.word("references"),
                "create" => self.word("create"),
                _ => self.ident(n.priv_name.clone()),
            }
        } else {
            self.word("all")
        }

        self.nbsp();

        if !n.cols.is_empty() {
            self.word("(");
            self.column_list(&n.cols);
            self.word(")");
        }
    }

    pub fn privilege_target(
        &mut self,
        targtype: &GrantTargetType,
        objtype: &ObjectType,
        objs: &[Node],
    ) {
        match targtype {
            GrantTargetType::AclTargetObject => match objtype {
                ObjectType::ObjectTable => self.print_list(objs),
                ObjectType::ObjectSequence => todo!(),
                ObjectType::ObjectFdw => todo!(),
                ObjectType::ObjectForeignServer => todo!(),
                ObjectType::ObjectFunction => todo!(),
                ObjectType::ObjectProcedure => todo!(),
                ObjectType::ObjectRoutine => todo!(),
                ObjectType::ObjectDatabase => todo!(),
                ObjectType::ObjectDomain => todo!(),
                ObjectType::ObjectLanguage => todo!(),
                ObjectType::ObjectLargeobject => todo!(),
                ObjectType::ObjectSchema => {
                    self.word("schema ");
                    self.name_list(objs);
                }
                ObjectType::ObjectTablespace => todo!(),
                ObjectType::ObjectType => todo!(),
                _ => {}
            },
            GrantTargetType::AclTargetAllInSchema => match objtype {
                ObjectType::ObjectTable => todo!(),
                ObjectType::ObjectSequence => todo!(),
                ObjectType::ObjectFunction => todo!(),
                ObjectType::ObjectProcedure => todo!(),
                ObjectType::ObjectRoutine => todo!(),
                _ => {}
            },
            GrantTargetType::AclTargetDefaults => match objtype {
                ObjectType::ObjectTable => todo!(),
                ObjectType::ObjectFunction => todo!(),
                ObjectType::ObjectSequence => todo!(),
                ObjectType::ObjectType => todo!(),
                ObjectType::ObjectSchema => todo!(),
                _ => {}
            },
            _ => {}
        }
    }
}
