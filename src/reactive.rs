use bevy::prelude::*;

#[bevy_trait_query::queryable]
#[reflect_trait]
pub trait Reactive: Reflect {
}