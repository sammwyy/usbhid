pub mod device;
pub mod keyboard;
pub mod mouse;

mod utils;

pub mod prelude {
    pub use super::device::*;
    pub use super::keyboard::*;
    pub use super::mouse::*;
}
