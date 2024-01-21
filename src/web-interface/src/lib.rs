mod bookmarks;
mod cookie;
mod error;
mod extern_type;
mod history;
mod notifications;
mod page_data;
mod tabs;
mod windows;

pub use bookmarks::*;
pub use cookie::*;
pub use error::*;
pub use extern_type::*;
pub use history::*;
pub use notifications::*;
pub use page_data::*;
pub use tabs::*;
pub use windows::*;

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
