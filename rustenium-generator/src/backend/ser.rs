use serde::{
    ser::SerializeSeq,
    Serializer,
};

use crate::backend::base_types::{Redirect, Variant};

pub fn serialize_usize<S>(n: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&n.to_string())
}

pub fn serialize_enum<S>(variants: &[Variant], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(variants.len()))?;

    for variant in variants {
        seq.serialize_element(variant.name.as_ref())?;
    }

    seq.end()
}

pub fn serialize_redirect<S>(redirect: &Option<Redirect>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(redirect) = redirect {
        if let Some(name) = redirect.name.as_ref() {
            return serializer.serialize_str(name.as_ref());
        }
    }
    serializer.serialize_none()
}

pub fn is_false(v: &bool) -> bool {
    !*v
}
