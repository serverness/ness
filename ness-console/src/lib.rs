pub mod api;
pub mod context;
pub mod error;
pub mod installs;
pub mod instances;

pub use crate::api::ConsoleApi;
pub use crate::context::Context;

pub enum ConsoleImpl {}

impl ConsoleApi for ConsoleImpl {
    type Context = Context;
}
