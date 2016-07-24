//! Text toolbox.
//!
//! ## Example
//!
//! ```
//! use text::Text;
//!
//! let text = Text::new("The quick brown fox jumps over the lazy dog.");
//! ```

extern crate font;

/// A text.
pub struct Text {
    #[allow(dead_code)]
    content: String,
}

impl Text {
    /// Create a text.
    pub fn new<T: Into<String>>(content: T) -> Text {
        Text { content: content.into() }
    }
}
