use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct GameConfig {
    autoexec: String,
    config: String,
}

impl fmt::Display for GameConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Autoexec         : {}\nConfig           : {}\n",
            self.autoexec, self.config
        )
    }
}
