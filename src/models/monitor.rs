use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Monitor {
    pub name: String,
    pub focused: bool,
}
