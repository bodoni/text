use font::Font;

use {Result, Text};

/// A layout.
pub struct Layout {
    #[allow(dead_code)]
    font: Font,
}

impl Layout {
    /// Create a layout.
    pub fn new(font: Font) -> Self {
        Layout { font: font }
    }

    /// Compose a text.
    pub fn draw<T: Into<String>>(&mut self, content: T) -> Result<Text> {
        Ok(Text::new(content))
    }
}
