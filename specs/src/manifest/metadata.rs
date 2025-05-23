use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub(crate) application: String,
    pub(crate) developer: String,
    pub(crate) diskspace: String,
    pub(crate) genres: Vec<String>,
    pub(crate) year: String
}
