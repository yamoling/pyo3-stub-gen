use itertools::Itertools;

use crate::{generate::*, type_info::*, TypeInfo};
use std::{
    collections::HashSet,
    fmt::{self},
};

/// Definition of a class member.
#[derive(Debug, Clone, PartialEq)]
pub struct MemberDef {
    pub is_property: bool,
    pub name: &'static str,
    pub r#type: TypeInfo,
    pub doc: &'static str,
}

impl Import for MemberDef {
    fn import(&self) -> HashSet<ModuleRef> {
        self.r#type.import.clone()
    }
}

impl From<&MemberInfo> for MemberDef {
    fn from(info: &MemberInfo) -> Self {
        Self {
            is_property: false,
            name: info.name,
            r#type: (info.r#type)(),
            doc: info.doc,
        }
    }
}

impl fmt::Display for MemberDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let indent = indent();
        let doc = self.doc.split("\n").join(&format!("\n{indent}"));
        if self.is_property {
            writeln!(f, "{indent}@property")?;
            writeln!(f, "{indent}def {}(self) -> {}:", self.name, self.r#type)?;
            if !doc.is_empty() {
                writeln!(f, r#"{indent}    """{doc}""""#)?;
            }
            writeln!(f, "{indent}    ...")
        } else {
            writeln!(f, "{indent}{}: {}", self.name, self.r#type)?;
            if !doc.is_empty() {
                writeln!(f, r#"{indent}"""{doc}""""#)?;
            }
            Ok(())
        }
    }
}
