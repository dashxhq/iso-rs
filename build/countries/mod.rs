use crate::{
    codegen::{
        country_struct, currency_struct, language_struct, map_builder::MapBuilder, vec_to_string,
    },
    countries::country_data::CountryData,
    hash_map_to_static, value_or_none, vec_or_none,
};
use proc_macro2::TokenStream;
use quote::quote;
use reqwest::blocking::get;
use serde_json::{from_str, Value};
use std::collections::HashMap;
use std::error::Error;

pub mod country_data;

const COUNTRY_DATASET: &'static str = "https://restcountries.com/v2/all";
type Regions<'a> = HashMap<&'a str, Vec<String>>;

pub fn get_countries() -> Result<TokenStream, Box<dyn Error>> {
    let data = get(COUNTRY_DATASET)?.text()?;
    let parsed: Value = from_str(data.as_str())?;
    let mut map = MapBuilder::new();
    let mut vec = Vec::new();
    let mut regions: Regions = HashMap::new();
    let mut capitals: Regions = HashMap::new();
    let mut alpha_2: Regions = HashMap::new();
    let mut alpha_3: Regions = HashMap::new();
    if let Some(x) = parsed.as_array() {
        for country in x.iter() {
            if let Some(country_data) = country.as_object() {
                let name = country_data.get("name");
                if let Some(country_name) = name {
                    vec.push(CountryData::new(
                        country_name.to_string(),
                        value_or_none!("capital", country_data),
                        value_or_none!("region", country_data),
                        value_or_none!("alpha2Code", country_data),
                        value_or_none!("alpha3Code", country_data),
                        vec_or_none!("timezones", country_data),
                        vec_or_none!("currencies", country_data, currencies),
                        vec_or_none!("languages", country_data, languages),
                        vec_or_none!("callingCodes", country_data),
                    ));
                };
            }
        }
    }

    for country_data in vec.iter() {
        let country = country_struct(country_data);

        let capital = country_data.capital.trim_matches('\"');
        let region = country_data.region.trim_matches('\"');
        if !capital.is_empty() {
            capitals.entry(capital).or_default().push(country.clone());
        }

        if !region.is_empty() {
            regions.entry(region).or_default().push(country.clone());
        }

        map.name()
            .entry(country_data.name.trim_matches('\"'), &country);
        alpha_2
            .entry(country_data.alpha_2.trim_matches('\"'))
            .or_default()
            .push(country.clone());
        alpha_3
            .entry(country_data.alpha_3.trim_matches('\"'))
            .or_default()
            .push(country.clone());
    }

    hash_map_to_static!(capitals, map, capital);
    hash_map_to_static!(regions, map, region);
    hash_map_to_static!(alpha_2, map, alpha_2);
    hash_map_to_static!(alpha_3, map, alpha_3);

    let parsed = map.parse()?;
    let names = parsed.name;
    let capital = parsed.capital;
    let regions = parsed.region;
    let alpha_2 = parsed.alpha_2;
    let alpha_3 = parsed.alpha_3;

    Ok(quote! {
        /// Map of all the countries with name as the key and value as [`Country`](struct.Country.html).
        pub static NAMES: phf::Map<&'static str, Country> = #names;
        #[cfg(feature = "from_capitals")]
        /// Map of all capitals with their countries as the value as an array of [`Country`](struct.Country.html).
        pub static CAPTIAL: phf::Map<&'static str, &'static [Country]> = #capital;
        #[cfg(feature = "from_regions")]
        /// Map of all regions with countries that reside in them.
        pub static REGIONS: phf::Map<&'static str, &'static [Country]> = #regions;
        #[cfg(feature = "from_alpha_2")]
        /// Map of all alpha_2 codes (key) with the corresponding countries as values.
        pub static ALPHA_2: phf::Map<&'static str, &'static [Country]> = #alpha_2;
        #[cfg(feature = "from_alpha_3")]
        /// Map of all alpha_3 codes (key) with the corresponding countries as values.
        pub static ALPHA_3: phf::Map<&'static str, &'static [Country]> = #alpha_3;
    })
}
