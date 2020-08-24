#![cfg_attr(not(feature = "std"), no_std)]

pub mod boolean;
pub mod splay;

#[cfg(not(feature = "std"))]
extern crate core as std;

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod lib {
    #[cfg(not(feature = "std"))]
    pub use alloc::vec::Vec;
    #[cfg(not(feature = "std"))]
    pub use alloc::{format, vec};
    #[cfg(feature = "std")]
    pub use std::vec::Vec;

    #[cfg(not(feature = "std"))]
    pub use alloc::boxed::Box;
    #[cfg(feature = "std")]
    pub use std::boxed::Box;

    #[cfg(not(feature = "std"))]
    pub use alloc::rc::{Rc, Weak};
    #[cfg(feature = "std")]
    pub use std::rc::{Rc, Weak};

    #[cfg(not(feature = "std"))]
    pub use alloc::collections::BinaryHeap;
    #[cfg(feature = "std")]
    pub use std::collections::{hash_set::HashSet, BinaryHeap};

    #[cfg(not(feature = "std"))]
    pub use hashbrown::HashSet;
}
