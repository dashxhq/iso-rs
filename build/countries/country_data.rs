#[derive(Debug)]
pub struct CountryData {
    pub name: String,
    pub capital: String,
    pub region: String,
    pub alpha_2: String,
    pub alpha_3: String,
    pub timezones: Vec<String>,
    pub currencies: Vec<String>,
    pub languages: Vec<String>,
    pub call_codes: Vec<String>,
}

impl CountryData {
    pub fn new(
        name: String,
        capital: String,
        region: String,
        alpha_2: String,
        alpha_3: String,
        timezones: Vec<String>,
        currencies: Vec<String>,
        languages: Vec<String>,
        call_codes: Vec<String>,
    ) -> Self {
        Self {
            name,
            capital,
            region,
            alpha_2,
            alpha_3,
            timezones,
            currencies,
            languages,
            call_codes,
        }
    }
}
