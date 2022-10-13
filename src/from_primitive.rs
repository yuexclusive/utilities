#![cfg(feature = "from_primitive")]

pub use num_traits::FromPrimitive;

pub mod derive {
    pub use num_derive::FromPrimitive;
}
