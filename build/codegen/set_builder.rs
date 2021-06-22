use crate::{map_method, tokens};
use phf_codegen::Set;
use proc_macro2::{LexError, TokenStream};
use std::str::FromStr;

pub struct SetBuilder<'a> {
    pub currencies: Set<&'a str>,
}

pub struct ParsedSet {
    pub currencies: TokenStream,
}

impl<'a> SetBuilder<'a> {
    pub fn new() -> Self {
        Self {
            currencies: Set::new(),
        }
    }

    pub fn currency(&mut self) -> &mut Set<&'a str> {
        &mut self.currencies
    }

    pub fn parse(self) -> Result<ParsedSet, LexError> {
        Ok(ParsedSet {
            currencies: tokens!(self.currencies),
        })
    }
}
