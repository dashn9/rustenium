#[cfg(feature = "serde0")]
use serde::{Deserialize, Serialize};

use std::borrow::Cow;

use crate::backend::types::ModuleDatatype;

#[cfg_attr(feature = "serde0", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Protocol<'a> {
    pub name: Option<&'a str>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing))]
    pub description: Option<Cow<'a, str>>,
    pub version: Version,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Vec::is_empty"))]
    pub modules: Vec<Module<'a>>,
}

#[cfg_attr(feature = "serde0", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Version {
    #[cfg_attr(feature = "serde0", serde(serialize_with = "ser::serialize_usize"))]
    pub major: usize,
    #[cfg_attr(feature = "serde0", serde(serialize_with = "ser::serialize_usize"))]
    pub minor: usize,
}


// Module(BiDi) & Domain(CDP) are use introspectively
#[cfg_attr(feature = "serde0", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module<'a> {
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<Cow<'a, str>>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "ser::is_false"))]
    pub experimental: bool,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "ser::is_false"))]
    pub deprecated: bool,
    #[cfg_attr(feature = "serde0", serde(rename = "domain"))]
    pub name: Cow<'a, str>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Vec::is_empty"))]
    pub dependencies: Vec<Cow<'a, str>>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Vec::is_empty"))]
    pub types: Vec<TypeDef<'a>>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Vec::is_empty"))]
    pub commands: Vec<Command<'a>>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Vec::is_empty"))]
    pub command_results: Vec<CommandResult<'a>>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Vec::is_empty"))]
    pub events: Vec<Event<'a>>,
}

#[cfg_attr(feature = "serde0", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeDef<'a> {
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<Cow<'a, str>>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "ser::is_false"))]
    pub experimental: bool,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "ser::is_false"))]
    pub deprecated: bool,
    pub name: Cow<'a, str>,
    #[cfg_attr(feature = "serde0", serde(flatten))]
    pub extends: Type<'a>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Vec::is_empty"))]
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

#[cfg_attr(feature = "serde0", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde0", serde(rename_all = "lowercase"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item<'a> {
    #[cfg_attr(feature = "serde0", serde(serialize_with = "ser::serialize_enum"))]
    Enum(Vec<Variant<'a>>),
    Properties(Vec<Param<'a>>),
}

#[cfg_attr(feature = "serde0", derive(Deserialize))]
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
    Ref(Cow<'a, str>),
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
                _ => Type::Ref(Cow::Borrowed(ty)),
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

#[cfg_attr(feature = "serde0", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant<'a> {
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Option::is_none"))]
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

#[cfg_attr(feature = "serde0", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param<'a> {
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<Cow<'a, str>>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "ser::is_false"))]
    pub experimental: bool,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "ser::is_false"))]
    pub deprecated: bool,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "ser::is_false"))]
    pub optional: bool,
    #[cfg_attr(feature = "serde0", serde(flatten))]
    pub r#type: Type<'a>,
    pub name: Cow<'a, str>,
    // RawType is the raw type.
    pub raw_name: Cow<'a, str>,
    // is_circular_dep indicates a type that causes circular dependencies.
    pub is_circular_dep: bool,
}

#[cfg_attr(feature = "serde0", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Command<'a> {
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<Cow<'a, str>>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "ser::is_false"))]
    pub experimental: bool,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "ser::is_false"))]
    pub deprecated: bool,
    pub name: Cow<'a, str>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde0", serde(serialize_with = "ser::serialize_redirect"))]
    pub redirect: Option<Redirect<'a>>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Vec::is_empty"))]
    pub parameters: Vec<Param<'a>>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Vec::is_empty"))]
    pub returns: Vec<&'a str>,
    // RawType is the raw type.
    pub raw_name: Cow<'a, str>,
    // is_circular_dep indicates a type that causes circular dependencies.
    pub is_circular_dep: bool,
}
#[cfg_attr(feature = "serde0", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandResult<'a> {
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<Cow<'a, str>>,
    pub name: Cow<'a, str>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Vec::is_empty"))]
    pub parameters: Vec<Param<'a>>,
    // RawType is the raw type.
    pub raw_name: Cow<'a, str>,
}
#[cfg_attr(feature = "serde0", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event<'a> {
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<Cow<'a, str>>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "ser::is_false"))]
    pub experimental: bool,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "ser::is_false"))]
    pub deprecated: bool,
    pub name: Cow<'a, str>,
    #[cfg_attr(feature = "serde0", serde(skip_serializing_if = "Vec::is_empty"))]
    pub parameters: Vec<Param<'a>>,
    // RawType is the raw type.
    pub raw_name: Cow<'a, str>,
    // IsCircularDep indicates a type that causes circular dependencies.
    pub is_circular_dep: bool,
}

#[cfg_attr(feature = "serde0", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Redirect<'a> {
    pub description: Option<Cow<'a, str>>,
    pub module: Cow<'a, str>,
    pub name: Option<Cow<'a, str>>,
}
