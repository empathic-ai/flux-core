#![allow(warnings)]
#![allow(unused)]

#[cfg(feature = "bevy")]
pub mod reactive;

pub mod prelude {
    #[cfg(feature = "bevy")]
    pub use crate::reactive::*;
}