use std::fmt;

use serde::{Deserialize, Serialize};

/// Represents metadata for a game.
///
/// This struct includes essential information about a game, such as its title,
/// publisher, release year, and an optional comment.
///
/// # Fields
/// - `title`: The title of the game.
/// - `publisher`: The name of the company or entity that published the game.
/// - `year`: The year the game was released.
/// - `comment`: An optional comment about the game, which may provide
///   additional context or information.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameMetadata {
    title: String,
    publisher: String,
    year: u32,
    comment: Option<String>,
}

impl fmt::Display for GameMetadata {
    /// Formats the game metadata into a human-readable string.
    ///
    /// This implementation aligns the fields for readability and substitutes
    /// "N/A" if no comment is provided.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title            : {}\nPublisher        : {}\nYear             : {}\nComment          : {}\n",
            self.title,
            self.publisher,
            self.year,
            self.comment.as_deref().unwrap_or("N/A")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::GameMetadata;

    /// Tests the `fmt::Display` implementation for `GameMetadata`.
    ///
    /// This test ensures that the game metadata is correctly formatted as
    /// a human-readable string.
    #[test]
    fn test_display() {
        let metadata = GameMetadata {
            title: "My Game".to_string(),
            publisher: "My Publisher".to_string(),
            year: 2025,
            comment: Some("Great game!".to_string()),
        };

        let expected = "Title            : My Game\nPublisher        : My Publisher\nYear             : 2025\nComment          : Great game!\n";
        assert_eq!(metadata.to_string(), expected);
    }
}
