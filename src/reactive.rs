#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{DynamicStruct, FromReflect, PartialReflect, Reflect, Typed, reflect_trait};

use ::serde::Serialize;

#[cfg(feature = "bevy_reflect")]
#[cfg_attr(feature = "bevy", bevy_trait_query::queryable)]
#[cfg_attr(feature = "bevy", reflect_trait)]
pub trait Reactive: Reflect {}

#[cfg(feature = "bevy_reflect")]
pub trait FromDynamic: Typed + FromReflect {
    fn from_dynamic(value: &DynamicStruct) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(type_info) = value.get_represented_type_info() {
            if type_info.type_path() == Self::type_path() {
                Self::from_reflect(value.as_partial_reflect())
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(feature = "bevy_reflect")]
impl<T: Typed + FromReflect> FromDynamic for T {}
