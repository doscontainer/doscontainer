use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Layer {
    url: String,
    checksum: Option<String>,
    label: Option<String>,
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "URL              : {}\nLabel            : {}\nChecksum         : {}\n",
            self.url,
            self.label.as_deref().unwrap_or("N/A"),
            self.checksum.as_deref().unwrap_or("N/A")
        )
    }
}

impl Layer {
    /// Creates a new `Layer` instance.
    ///
    /// # Arguments
    /// - `url`: The URL for the layer.
    /// - `label`: An optional label for the layer.
    /// - `checksum`: An optional checksum for the layer.
    ///
    /// # Returns
    /// A new `Layer` instance.
    pub fn new(url: &str, label: Option<&str>, checksum: Option<&str>) -> Self {
        Layer {
            url: url.to_string(),
            label: label.map(|l| l.to_string()),
            checksum: checksum.map(|c| c.to_string()),
        }
    }

    /// Returns the URL of the layer.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the label of the layer, or the URL if no label is present.
    pub fn label(&self) -> String {
        self.label.clone().unwrap_or_else(|| self.url.clone())
    }

    /// Returns the checksum of the layer, if available.
    pub fn checksum(&self) -> Option<&str> {
        self.checksum.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::Layer;

    /// Tests the `Layer::new` method and its accessors.
    #[test]
    fn test_layer_creation() {
        let layer = Layer::new("http://example.com", Some("Example Label"), Some("12345"));

        assert_eq!(layer.url(), "http://example.com");
        assert_eq!(layer.label(), "Example Label");
        assert_eq!(layer.checksum(), Some("12345"));
    }

    /// Tests the `fmt::Display` implementation for `Layer`.
    #[test]
    fn test_display() {
        let layer = Layer::new("http://example.com", None, Some("12345"));

        let expected = "URL              : http://example.com\nLabel            : N/A\nChecksum         : 12345\n";
        assert_eq!(layer.to_string(), expected);
    }

    /// Tests fallback behavior for `label` when it is not set.
    #[test]
    fn test_label_fallback() {
        let layer = Layer::new("http://example.com", None, None);

        assert_eq!(layer.label(), "http://example.com");
    }
}
