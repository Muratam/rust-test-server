use prpr::*;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
#[macro_use]
extern crate assert_type_eq;

// for entry_point
pub use wasm_bindgen::prelude::wasm_bindgen as entry_point;
pub extern crate wasm_bindgen;

// for publish
pub mod js;
pub mod prgl;
pub mod prhtml;
pub mod sample;
pub mod system;
pub use prhtml::traits::*;
pub use system::{input, NeedUpdate, Time, Updater};
pub use system::{Why, WhyTrait, Whys};
