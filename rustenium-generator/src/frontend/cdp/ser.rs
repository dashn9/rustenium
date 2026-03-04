use serde::{
    ser::SerializeMap,
    Serialize, Serializer,
};

use crate::backend::base_types::{Protocol, Type};

impl Protocol<'_> {
    /// Serialize the `Protocol` data structure as a String of JSON.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// Serialize the `Protocol` data structure as a pretty-printed String of
    /// JSON.
    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

impl Serialize for Type<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        match self {
            Type::Integer => {
                map.serialize_entry("type", "integer")?;
            }
            Type::Number => {
                map.serialize_entry("type", "number")?;
            }
            Type::Boolean => {
                map.serialize_entry("type", "boolean")?;
            }
            Type::String => {
                map.serialize_entry("type", "string")?;
            }
            Type::Object => {
                map.serialize_entry("type", "object")?;
            }
            Type::Any => {
                map.serialize_entry("type", "any")?;
            }
            Type::Binary => {
                map.serialize_entry("type", "binary")?;
            }
            Type::Enum(variants) => {
                map.serialize_entry("type", "string")?;
                map.serialize_entry(
                    "enum",
                    &variants
                        .iter()
                        .map(|variant| variant.name.as_ref())
                        .collect::<Vec<_>>(),
                )?;
            }
            Type::ArrayOf(ty) => {
                map.serialize_entry("type", "array")?;
                map.serialize_entry("items", &ty)?;
            }
            Type::Ref(type_ref) => {
                // Serialize back to the original dot-separated form
                let ref_str = if let Some(ref module) = type_ref.module {
                    format!("{}.{}", module, type_ref.name)
                } else {
                    type_ref.name.to_string()
                };
                map.serialize_entry("$ref", &ref_str)?;
            }
        }

        map.end()
    }
}
