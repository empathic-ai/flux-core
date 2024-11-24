#![allow(warnings)]
#![allow(unused)]

pub mod thing;
pub mod task;

pub mod prelude {
    pub use crate::task::*;
    pub use crate::thing::*;
}