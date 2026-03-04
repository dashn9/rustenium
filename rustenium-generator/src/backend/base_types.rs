use serde::Serialize;

use std::borrow::Cow;

use crate::backend::types::ModuleDatatype;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Protocol<'a> {
    pub name: Option<&'a str>,
    #[serde(skip_serializing)]
    pub description: Option<Cow<'a, str>>,
    pub version: Version,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub modules: Vec<Module<'a>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Version {
    #[serde(serialize_with = "super::ser::serialize_usize")]
    pub major: usize,
    #[serde(serialize_with = "super::ser::serialize_usize")]
    pub minor: usize,
}


// Module(BiDi) & Domain(CDP) are use introspectively
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Module<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "super::ser::is_false")]
    pub experimental: bool,
    #[serde(skip_serializing_if = "super::ser::is_false")]
    pub deprecated: bool,
    #[serde(rename = "domain")]
    pub name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<TypeDef<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<Command<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub command_results: Vec<CommandResult<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<Event<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TypeDef<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "super::ser::is_false")]
    pub experimental: bool,
    #[serde(skip_serializing_if = "super::ser::is_false")]
    pub deprecated: bool,
    pub name: Cow<'a, str>,
    #[serde(flatten)]
    pub extends: Type<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Item<'a>>,
    // RawType is the raw type.
    pub raw_name: Cow<'a, str>,
    // is_circular_dep indicates a type that causes circular dependencies.
    pub is_circular_dep: bool,
}

impl<'a> TypeDef<'a> {
    pub fn is_enum(&self) -> bool {
        matches!(self.parameters.as_ref(), Some(Item::Enum(_)))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Item<'a> {
    #[serde(serialize_with = "super::ser::serialize_enum")]
    Enum(Vec<Variant<'a>>),
    Properties(Vec<Param<'a>>),
}

/// A structured reference to a type, possibly in another module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeRef<'a> {
    /// The referenced module name, if cross-module (e.g., "Runtime")
    pub module: Option<Cow<'a, str>>,
    /// The type name (e.g., "ScriptId")
    pub name: Cow<'a, str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type<'a> {
    Integer,
    Number,
    Boolean,
    String,
    Object,
    Any,
    Binary,
    Enum(Vec<Variant<'a>>),
    ArrayOf(Box<Type<'a>>),
    Ref(TypeRef<'a>),
}

impl Type<'_> {
    pub(crate) fn new(ty: &str, is_array: bool) -> Type<'_> {
        if is_array {
            Type::ArrayOf(Box::new(Type::new(ty, false)))
        } else {
            match ty {
                "enum" => Type::Enum(vec![]),
                "integer" => Type::Integer,
                "number" => Type::Number,
                "boolean" => Type::Boolean,
                "string" => Type::String,
                "object" => Type::Object,
                "any" => Type::Any,
                "binary" => Type::Binary,
                _ => {
                    if let Some((module, name)) = ty.split_once('.') {
                        Type::Ref(TypeRef {
                            module: Some(Cow::Borrowed(module)),
                            name: Cow::Borrowed(name),
                        })
                    } else {
                        Type::Ref(TypeRef {
                            module: None,
                            name: Cow::Borrowed(ty),
                        })
                    }
                }
            }
        }
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, Type::Enum(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Type::String)
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Type::Integer)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Variant<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    pub name: Cow<'a, str>,
}

impl<'a> Variant<'a> {
    pub fn new(name: &str) -> Variant<'_> {
        Variant {
            description: Default::default(),
            name: Cow::Borrowed(name),
        }
    }

    pub fn from(dt: &'a ModuleDatatype) -> Variant<'a>{
        Variant {
            description: dt.description().map(Cow::Borrowed),
            name: Cow::Borrowed(dt.name()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Param<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "super::ser::is_false")]
    pub experimental: bool,
    #[serde(skip_serializing_if = "super::ser::is_false")]
    pub deprecated: bool,
    #[serde(skip_serializing_if = "super::ser::is_false")]
    pub optional: bool,
    #[serde(flatten)]
    pub r#type: Type<'a>,
    pub name: Cow<'a, str>,
    // RawType is the raw type.
    pub raw_name: Cow<'a, str>,
    // is_circular_dep indicates a type that causes circular dependencies.
    pub is_circular_dep: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CommandMethod<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "super::ser::is_false")]
    pub experimental: bool,
    #[serde(skip_serializing_if = "super::ser::is_false")]
    pub deprecated: bool,
    pub name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "super::ser::serialize_redirect")]
    pub redirect: Option<Redirect<'a>>,
    /// The protocol method identifier, e.g. "Runtime.evaluate"
    pub raw_name: Cow<'a, str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Command<'a> {
    pub method: CommandMethod<'a>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<Param<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub returns: Vec<&'a str>,
    pub is_circular_dep: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CommandResult<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    pub name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<Param<'a>>,
    // RawType is the raw type.
    pub raw_name: Cow<'a, str>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Event<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "super::ser::is_false")]
    pub experimental: bool,
    #[serde(skip_serializing_if = "super::ser::is_false")]
    pub deprecated: bool,
    pub name: Cow<'a, str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<Param<'a>>,
    // RawType is the raw type.
    pub raw_name: Cow<'a, str>,
    // IsCircularDep indicates a type that causes circular dependencies.
    pub is_circular_dep: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Redirect<'a> {
    pub description: Option<Cow<'a, str>>,
    pub module: Cow<'a, str>,
    pub name: Option<Cow<'a, str>>,
}
