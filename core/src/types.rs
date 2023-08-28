use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Ident(pub String);

impl Ident {
    pub fn new(i: &str) -> Result<Self> {
        Ok(Self(i.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Literal {
    Hex(String),
    String(String),
    Int(String),
}

lazy_static! {
    static ref HEX_LITERAL: Regex = Regex::new(r"^[0-9A-Fa-f]+$").unwrap();
    static ref INT_LITERAL: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
}

impl Literal {
    pub fn hex(s: &str) -> Result<Self> {
        if s.len() % 2 == 0 && HEX_LITERAL.is_match(s) {
            Ok(Literal::Hex(String::from(s)))
        } else {
            Err(anyhow!("Wrong format of hex string"))
        }
    }

    pub fn string(s: &str) -> Result<Self> {
        Ok(Self::String(String::from(s)))
    }

    pub fn int(s: &str) -> Result<Self> {
        if INT_LITERAL.is_match(s) {
            Ok(Self::Int(String::from(s)))
        } else {
            Err(anyhow!("Wrong format of int string"))
        }
    }
}
