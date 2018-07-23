extern crate geo_types;

mod consts;
mod private;

mod codearea;
pub use codearea::CodeArea;

mod interface;
pub use interface::{decode, encode, is_full, is_short, is_valid, recover_nearest, shorten};
