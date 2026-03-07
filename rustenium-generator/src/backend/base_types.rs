use std::borrow::Cow;

use crate::backend::types::ModuleDatatype;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DomainDirection { Remote, Local, All }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Protocol<'a> {
    pub name: Option<&'a str>,
    pub description: Option<Cow<'a, str>>,
    pub version: Version,
    pub modules: Vec<Module<'a>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Version {
    pub major: usize,
    pub minor: usize,
}

// Module(BiDi) & Domain(CDP) are use introspectively
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Module<'a> {
    pub description: Option<Cow<'a, str>>,
    pub experimental: bool,
    pub deprecated: bool,
    pub name: Cow<'a, str>,
    pub dependencies: Vec<Cow<'a, str>>,
    pub types: Vec<TypeDef<'a>>,
    pub commands: Vec<Command<'a>>,
    pub command_results: Vec<CommandResult<'a>>,
    pub events: Vec<Event<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeDef<'a> {
    pub description: Option<Cow<'a, str>>,
    pub experimental: bool,
    pub deprecated: bool,
    pub name: Cow<'a, str>,
    pub extends: Type<'a>,
    pub parameters: Option<Item<'a>>,
    pub raw_name: Cow<'a, str>,
    pub is_circular_dep: bool,
    pub direction: Option<DomainDirection>,
}

impl<'a> TypeDef<'a> {
    pub fn is_enum(&self) -> bool {
        matches!(self.parameters.as_ref(), Some(Item::Enum(_)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Item<'a> {
    Enum(Vec<Variant<'a>>),
    Properties(Vec<Param<'a>>),
    /// Untagged type choice: `( TypeA // TypeB )` — each variant wraps a type ref.
    TypeChoice(Vec<TypeRef<'a>>),
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
    UnsignedInteger,
    Number,
    Boolean,
    String,
    Object,
    Any,
    Binary,
    Enum(Vec<Variant<'a>>),
    ArrayOf(Box<Type<'a>>),
    Ref(TypeRef<'a>),
    /// HashMap<String, serde_json::Value> with #[serde(flatten)]
    Extensible,
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
        matches!(self, Type::Integer | Type::UnsignedInteger)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant<'a> {
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

    pub fn from(dt: &'a ModuleDatatype) -> Variant<'a> {
        Variant {
            description: dt.description().map(Cow::Borrowed),
            name: Cow::Borrowed(dt.name()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Constraint {
    Ge(f64),
    Gt(f64),
    Le(f64),
    Lt(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Param<'a> {
    pub description: Option<Cow<'a, str>>,
    pub experimental: bool,
    pub deprecated: bool,
    pub optional: bool,
    pub r#type: Type<'a>,
    pub name: Cow<'a, str>,
    pub raw_name: Cow<'a, str>,
    pub is_circular_dep: bool,
    pub default_value: Option<String>,
    pub validation: Option<Vec<Constraint>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandMethod<'a> {
    pub description: Option<Cow<'a, str>>,
    pub experimental: bool,
    pub deprecated: bool,
    pub name: Cow<'a, str>,
    pub redirect: Option<Redirect<'a>>,
    pub raw_name: Cow<'a, str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Command<'a> {
    pub method: CommandMethod<'a>,
    pub params: Vec<Param<'a>>,
    pub returns: Vec<&'a str>,
    pub is_circular_dep: bool,
    pub direction: Option<DomainDirection>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandResult<'a> {
    pub description: Option<Cow<'a, str>>,
    pub name: Cow<'a, str>,
    pub parameters: Vec<Param<'a>>,
    pub raw_name: Cow<'a, str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event<'a> {
    pub description: Option<Cow<'a, str>>,
    pub experimental: bool,
    pub deprecated: bool,
    pub name: Cow<'a, str>,
    pub parameters: Vec<Param<'a>>,
    pub raw_name: Cow<'a, str>,
    pub is_circular_dep: bool,
    pub direction: Option<DomainDirection>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Redirect<'a> {
    pub description: Option<Cow<'a, str>>,
    pub module: Cow<'a, str>,
    pub name: Option<Cow<'a, str>>,
}
