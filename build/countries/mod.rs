use crate::time::Timezones;
use crate::{
    codegen::{
        country_struct, currency_struct, language_struct, map_builder::MapBuilder, vec_to_string,
    },
    countries::country_data::CountryData,
    hash_map_to_static,
    time::timezone_vec,
    value_or_none, vec_or_none,
};
use proc_macro2::TokenStream;
use quote::quote;

use serde_json::{from_str, Value};
use std::collections::HashMap;

use std::error::Error;
use std::fs::File;
use std::io::Read;

pub mod country_data;

type ItemsMap<'a> = HashMap<&'a str, Vec<String>>;

pub fn get_countries(timezones: Timezones) -> Result<TokenStream, Box<dyn Error>> {
    let mut countries =
        File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/build/countries.json")).unwrap();
    let mut data = String::new();
    countries.read_to_string(&mut data).unwrap();
    let parsed: Value = from_str(data.as_str())?;
    let mut map = MapBuilder::new();
    let mut vec: Vec<CountryData> = Vec::new();
    let mut regions: ItemsMap = HashMap::new();
    let mut capitals: ItemsMap = HashMap::new();
    let mut alpha_2: ItemsMap = HashMap::new();
    let mut alpha_3: ItemsMap = HashMap::new();
    if let Some(x) = parsed.as_array() {
        for country in x.iter() {
            if let Some(country_data) = country.as_object() {
                let name = country_data.get("name");
                if let Some(country_name) = name {
                    if !vec.iter().any(|data| {
                        data.name.trim_matches('\"') == country_name.to_string().trim_matches('\"')
                    }) {
                        let alpha_2 = value_or_none!("alpha2Code", country_data);
                        let zone = timezones.get(&alpha_2);
                        vec.push(
                            CountryData::builder()
                                .name(country_name.to_string())
                                .capital(value_or_none!("capital", country_data))
                                .region(value_or_none!("region", country_data))
                                .alpha_2(alpha_2)
                                .alpha_3(value_or_none!("alpha3Code", country_data))
                                .timezones(timezone_vec(
                                    zone.cloned().unwrap_or_else(Vec::new).to_vec(),
                                ))
                                .currencies(vec_or_none!("currencies", country_data, currencies))
                                .languages(vec_or_none!("languages", country_data, languages))
                                .call_codes(vec_or_none!("callingCodes", country_data))
                                .build(),
                        );
                    }
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

    let parsed_map = map.parse()?;
    let names = parsed_map.name;
    let capital = parsed_map.capital;
    let regions = parsed_map.region;
    let alpha_2 = parsed_map.alpha_2;
    let alpha_3 = parsed_map.alpha_3;

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
