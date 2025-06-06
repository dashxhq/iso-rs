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
    pub fn builder() -> CountryDataBuilder {
        CountryDataBuilder::default()
    }
}

#[derive(Default, Debug)]
pub struct CountryDataBuilder {
    name: String,
    capital: String,
    region: String,
    alpha_2: String,
    alpha_3: String,
    timezones: Vec<String>,
    currencies: Vec<String>,
    languages: Vec<String>,
    call_codes: Vec<String>,
}

impl CountryDataBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn capital(mut self, capital: String) -> Self {
        self.capital = capital;
        self
    }

    pub fn region(mut self, region: String) -> Self {
        self.region = region;
        self
    }

    pub fn alpha_2(mut self, alpha_2: String) -> Self {
        self.alpha_2 = alpha_2;
        self
    }

    pub fn alpha_3(mut self, alpha_3: String) -> Self {
        self.alpha_3 = alpha_3;
        self
    }

    pub fn timezones(mut self, timezones: Vec<String>) -> Self {
        self.timezones = timezones;
        self
    }

    pub fn currencies(mut self, currencies: Vec<String>) -> Self {
        self.currencies = currencies;
        self
    }

    pub fn languages(mut self, languages: Vec<String>) -> Self {
        self.languages = languages;
        self
    }

    pub fn call_codes(mut self, call_codes: Vec<String>) -> Self {
        self.call_codes = call_codes;
        self
    }

    pub fn build(self) -> CountryData {
        CountryData {
            name: self.name,
            capital: self.capital,
            region: self.region,
            alpha_2: self.alpha_2,
            alpha_3: self.alpha_3,
            timezones: self.timezones,
            currencies: self.currencies,
            languages: self.languages,
            call_codes: self.call_codes,
        }
    }
}
