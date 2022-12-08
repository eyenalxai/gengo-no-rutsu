use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct PrefixData{
    pub base: String,
    pub target: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Loan {
    pub non_native: String,
    pub native: Vec<Vec<String>>,
    pub inexact: String,
    pub extra_normal_form: String,
    pub unrecognized_forms: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Root<'data> {
    Native(String),
    Loanword(&'data Loan),
}

#[derive(Debug, PartialEq, Eq)]
pub struct WordData<'data> (pub &'data PrefixData, pub Root<'data>);

impl Display for WordData<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let prefix = if self.0.target == "" { &self.0.base } else { &self.0.target };

        match &self.1 {
            Root::Native(root) => write!(f, "Не {}{}, а {}{}.", self.0.base, root, self.0.target, root),
            Root::Loanword(root) => {
                let mut output_native = String::new();

                let meaning_blocks = root.native.len() > 1;
                for i in 0..root.native.len(){
                    let meaning = &root.native[i];
                    if meaning_blocks {
                        output_native += &(i+1).to_string();
                        output_native += ") ";
                    }
                    let mut first = true;
                    for word in meaning.iter() {
                        if first {
                            first = false;
                        } else {
                            output_native += ", ";
                        }
                        output_native += prefix;
                        output_native += &word;
                    }

                    if meaning_blocks && i+1 < root.native.len() {
                        output_native += " ";
                    }
                }
                if !root.inexact.is_empty() {
                    return write!(
                        f,
                        "Если вы имели в виду не {}, то будет правильно {}",
                        root.inexact, output_native
                    );
                }
                write!(f, "Не {}{}, а {}.", self.0.base, root.non_native, output_native)
            }
        }
    }
}