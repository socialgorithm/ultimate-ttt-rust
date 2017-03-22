#[path = "uttt/mod.rs"]
mod inner_uttt;

#[path = "ttt/mod.rs"]
mod inner_ttt;

pub mod uttt {
    pub use inner_uttt::*;
    pub use inner_ttt::*;
}