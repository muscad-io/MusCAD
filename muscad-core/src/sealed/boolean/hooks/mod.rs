use super::*;

#[macro_use]
mod macros;

mod conf;
pub use conf::DebugHooks;

impl<T: Float> std::fmt::Debug for DebugHooks<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<DebugHooks>")
    }
}
