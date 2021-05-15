#![forbid(unsafe_code)]
//! `iso-rs` crate provides methods to extract [ISO 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1) (codes for country and dependent area names)
//! [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639#Alpha-2_code_space) (Alpha-2 code), [ISO 639-2](https://en.wikipedia.org/wiki/ISO_639#Alpha-3_code_space) (Alpha-3 code) (Codes for the representation of names of languages)
//! codes, timezones, captials, regions, subregions, [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217) currency codes, etc. for all countries.
//!
//! `iso-rs` crate is powered by the [REST Countries API](https://gitlab.com/amatos/rest-countries/)
//! If you find this library useful and would like to show your gratitude, consider
//! donating to the restcountries project.
//!
//! The crate allows you to query for country data by various fields.
//! It pulls the data from the restcountries API and generates a compile-time
//! static map using [phf-codegen](https://docs.rs/phf_codegen/0.8.0/phf_codegen/). The methods just query these maps.
//! You can query countries by their name, capital (enable feature), etc.
//!
//! # Features
//!
//! - `from_capitals`: Allows you to query country data by country capitals.
//! - `from_alpha_2`: Allows you to query country data by alpha_2 codes.
//! - `from_alpha_3`: Allows you to query country data by alpha_3 codes.
//! - `from_regions`: Allows you to query country data by their regions.
//!
//! By default all these features are enabled. It is recommended to
//! turn off the features you will not be using as the country data is
//! high in number and you'll be saving some static allocation.
//!
//! # Example
//!
//! ```
//! use iso_rs::prelude::*;
//!
//! let country = Country::from_name("India").unwrap();
//! assert_eq!(country.capital.unwrap(), "New Delhi");
//! ```
//!

/// Prelude brings the `Country`, `Currency` and `Language` structs in scope.
pub mod prelude {
    pub use crate::{Country, Currency, Language};
}

/// Represents a Country.
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct Country {
    /// Name of the country, eg. "United States".
    pub name: &'static str,
    /// Name of the country's capital, eg. "Washington, DC".
    pub capital: Option<&'static str>,
    /// [Region](https://gitlab.com/amatos/rest-countries#continent) of the country
    pub region: Option<&'static str>,
    /// ISO 3166-1 2-letter country code
    pub alpha_2: &'static str,
    /// 3166-1 3-letter country code
    pub alpha_3: &'static str,
    /// Timezones that country has in UTC, eg. `UTC-05:00` for columbia
    pub timezones: &'static [&'static str],
    /// Currencies used in the country
    pub currencies: &'static [Currency],
    /// Languages used in the country
    pub languages: &'static [Language],
    /// Dialling codes used in a country
    pub call_codes: &'static [&'static str],
}

/// Represents a Currency with ISO 4217 code.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Currency {
    /// ISO 4217 currency code
    pub code: Option<&'static str>,
    /// Name of the currency in english
    pub name: Option<&'static str>,
    /// Symbol of the currency
    pub symbol: Option<&'static str>,
}

/// Represents a Language with both ISO 639-1 and ISO 639-2 codes.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Language {
    /// ISO 639-1 language code
    pub iso639_1: Option<&'static str>,
    /// ISO 639-2 language code
    pub iso639_2: Option<&'static str>,
    /// Name of the language in english
    pub name: Option<&'static str>,
    /// Native name of the language, can be in the native language
    pub native_name: Option<&'static str>,
}

// Generated code
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

impl Country {
    /// Get the country from it's name
    ///
    /// # Example
    ///
    /// ```
    /// use iso_rs::prelude::*;
    ///
    /// let country = Country::from_name("India").unwrap();
    /// assert_eq!(country.capital.unwrap(), "New Delhi");
    /// ```
    pub fn from_name(name: &str) -> Option<&'static Self> {
        NAMES.get(name)
    }
    /// Get a list of countries from a capital
    ///
    /// # Example
    ///
    /// ```
    /// use iso_rs::prelude::*;
    ///
    /// let country = Country::from_capital("New Delhi").unwrap()[0];
    /// assert_eq!(country.name, "India");
    /// ```
    #[cfg(feature = "from_capitals")]
    pub fn from_capital(capital: &str) -> Option<&'static [Self]> {
        CAPTIAL.get(capital).map(|e| *e)
    }
    /// Get a list of countries inside a region
    ///
    /// # Example
    ///
    /// ```
    /// use iso_rs::prelude::*;
    ///
    /// let southern_asia = Country::from_region("Southern Asia").unwrap();
    /// assert!(southern_asia.contains(Country::from_name("India").unwrap()));
    /// ```
    #[cfg(feature = "from_regions")]
    pub fn from_region(region: &str) -> Option<&'static [Self]> {
        REGIONS.get(region).map(|e| *e)
    }
    /// Get the country from its ISO 3166-1 alpha_2 code
    ///
    /// # Example
    ///
    /// ```
    /// use iso_rs::prelude::*;
    ///
    /// let mut country = Country::from_alpha_2("IN").unwrap();
    /// assert_eq!(country[0], *Country::from_name("India").unwrap());
    /// ```
    #[cfg(feature = "from_alpha_2")]
    pub fn from_alpha_2(alpha_2: &str) -> Option<&'static [Self]> {
        ALPHA_2.get(alpha_2).map(|e| *e)
    }
    /// Get the country from its ISO 3166-1 alpha_3 code
    ///
    /// # Example
    ///
    /// ```
    /// use iso_rs::prelude::*;
    ///
    /// let mut country = Country::from_alpha_3("IND").unwrap();
    /// assert_eq!(country[0], *Country::from_name("India").unwrap());
    /// ```
    #[cfg(feature = "from_alpha_3")]
    pub fn from_alpha_3(alpha_3: &str) -> Option<&'static [Self]> {
        ALPHA_3.get(alpha_3).map(|e| *e)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! india_check {
        ( $india : expr ) => {
            assert_eq!($india.capital.unwrap(), "New Delhi");
            assert_eq!($india.region.unwrap(), "Southern Asia");
            assert_eq!($india.alpha_2, "IN");
            assert_eq!($india.alpha_3, "IND");
            assert_eq!($india.timezones[0], "UTC+05:30");
            assert_eq!($india.call_codes[0], "91");
            assert_eq!(
                $india.currencies[0],
                Currency {
                    code: Some("INR"),
                    name: Some("Indian rupee"),
                    symbol: Some("₹"),
                }
            );
            assert_eq!(
                $india.languages[0],
                Language {
                    iso639_1: Some("hi"),
                    iso639_2: Some("hin"),
                    name: Some("Hindi"),
                    native_name: Some("हिन्दी"),
                }
            );
            assert_eq!(
                $india.languages[1],
                Language {
                    iso639_1: Some("en"),
                    iso639_2: Some("eng"),
                    name: Some("English"),
                    native_name: Some("English"),
                }
            );
        };
    }

    #[test]
    fn basic_country_fetching_from_name() {
        let india = Country::from_name("India").unwrap();
        india_check!(india);
    }

    #[cfg(feature = "from_capitals")]
    #[test]
    fn basic_country_fetching_from_capital() {
        let india = Country::from_capital("New Delhi").unwrap();
        india_check!(india[0]);
    }

    #[cfg(feature = "from_alpha_3")]
    #[test]
    fn basic_country_fetching_from_alpha_2() {
        let india = Country::from_alpha_2("IN").unwrap()[0];
        india_check!(india);
    }

    #[cfg(feature = "from_alpha_3")]
    #[test]
    fn basic_country_fetching_from_alpha_3() {
        let india = Country::from_alpha_3("IND").unwrap()[0];
        india_check!(india);
    }

    #[cfg(feature = "from_regions")]
    #[test]
    fn basic_country_fetching_from_region() {
        let southern_asia = Country::from_region("Southern Asia").unwrap();
        assert!(southern_asia.contains(Country::from_name("India").unwrap()));
    }
}
