/// A text.
pub struct Text {
    #[allow(dead_code)]
    content: String,
}

impl Text {
    /// Create a text.
    pub fn new<T: Into<String>>(content: T) -> Self {
        Text { content: content.into() }
    }
}
