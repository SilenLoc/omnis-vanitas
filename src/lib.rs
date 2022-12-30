#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod outcalls;
pub use app::native::App;
pub use outcalls::check_ready;
