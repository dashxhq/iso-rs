use crate::{map_method, tokens};
use phf_codegen::Map;
use proc_macro2::{LexError, TokenStream};
use std::str::FromStr;

pub struct MapBuilder<'a> {
    pub name: Map<&'a str>,
    pub capital: Map<&'a str>,
    pub region: Map<&'a str>,
    pub alpha_2: Map<&'a str>,
    pub alpha_3: Map<&'a str>,
}

pub struct ParsedMap {
    pub name: TokenStream,
    pub capital: TokenStream,
    pub region: TokenStream,
    pub alpha_2: TokenStream,
    pub alpha_3: TokenStream,
}

impl<'a> MapBuilder<'a> {
    pub fn new() -> Self {
        Self {
            name: Map::new(),
            capital: Map::new(),
            region: Map::new(),
            alpha_2: Map::new(),
            alpha_3: Map::new(),
        }
    }

    // helper methods to get mutable references fast
    map_method!(name);
    map_method!(capital);
    map_method!(region);
    map_method!(alpha_2);
    map_method!(alpha_3);

    pub fn parse(self) -> Result<ParsedMap, LexError> {
        Ok(ParsedMap {
            name: tokens!(self.name),
            capital: tokens!(self.capital),
            region: tokens!(self.region),
            alpha_2: tokens!(self.alpha_2),
            alpha_3: tokens!(self.alpha_3),
        })
    }
}
