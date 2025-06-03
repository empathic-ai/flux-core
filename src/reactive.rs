use bevy::{prelude::*, reflect::{DynamicStruct, Typed}};

#[bevy_trait_query::queryable]
#[reflect_trait]
pub trait Reactive: Reflect {

}

pub trait FromDynamic: Typed + FromReflect {
	fn from_dynamic(value: &DynamicStruct) -> Option<Self> where Self: Sized {
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

impl<T: Typed + FromReflect> FromDynamic for T {

}