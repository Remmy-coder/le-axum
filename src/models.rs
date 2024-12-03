use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CodeInput {
    pub code: String,
}

#[derive(Serialize)]
pub struct CodeOutput {
    pub output: String,
}
