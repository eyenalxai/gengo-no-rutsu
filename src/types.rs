use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum PollingMode {
    Polling,
    Webhook,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Words {
    pub native: String,
    pub non_native: String,
    // pub exclusions: String,
    // pub inexact: String,
    pub extra_normal_form: String,
    // pub unrecognized_forms: String,
}
