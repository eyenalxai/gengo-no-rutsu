use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use strsim::normalized_damerau_levenshtein;

#[derive(Debug, Clone, Copy)]
pub enum PollingMode {
    Polling,
    Webhook,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Word {
    pub non_native: String,
    pub native: String,
    pub inexact: String,
    pub extra_normal_form: String,
    pub unrecognized_forms: String,
}

pub trait Check {
    fn is_non_native(&self, word: String) -> bool;
}
