use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Hardware {
    pub graphics: Vec<String>,
    pub sound: Vec<String>,
}

impl Default for Hardware {
    fn default() -> Self {
        Hardware {
            graphics: Vec::new(),
            sound: Vec::new(),
        }
    }
}

impl Hardware {
    pub fn graphics(&self) -> &[String] {
        &self.graphics
    }

    pub fn sound(&self) -> &[String] {
        &self.sound
    }
}
