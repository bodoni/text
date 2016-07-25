//! Text toolbox.
//!
//! ## Example
//!
//! ```
//! extern crate font;
//! extern crate text;
//!
//! use font::File;
//! use text::Layout;
//!
//! # fn main() {
//! let path = "SourceSerifPro-Regular.otf";
//! # let path = "tests/fixtures/SourceSerifPro-Regular.otf";
//! let font = File::open(path).unwrap().fonts.remove(0);
//!
//! let mut layout = Layout::new(font);
//! let text = layout.draw("The quick brown fox jumps over the lazy dog.").unwrap();
//! # }
//! ```

extern crate font;

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::io::Result<T>;

mod layout;
mod text;

pub use layout::Layout;
pub use text::Text;
