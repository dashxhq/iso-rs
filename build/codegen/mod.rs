use crate::countries::country_data::CountryData;
use crate::field_entry;
use serde_json::Value;
use std::fmt::Display;

pub mod map_builder;

pub fn country_struct(country_data: &CountryData) -> String {
    let mut struct_string = String::from("Country {");
    let mut struct_values = String::new();
    let name = &country_data.name;
    let capital = &country_data.capital;
    let region = &country_data.region;
    let alpha_2 = &country_data.alpha_2;
    let alpha_3 = &country_data.alpha_3;
    let timezones = vec_to_string(&country_data.timezones);
    let currencies = vec_to_string(&country_data.currencies);
    let languages = vec_to_string(&country_data.languages);
    let call_codes = vec_to_string(&country_data.call_codes);
    field_entry!(struct_values, name);
    field_entry!(struct_values, Some(capital));
    field_entry!(struct_values, Some(region));
    field_entry!(struct_values, alpha_2);
    field_entry!(struct_values, alpha_3);
    field_entry!(struct_values, timezones);
    field_entry!(struct_values, currencies);
    field_entry!(struct_values, languages);
    field_entry!(struct_values, call_codes);
    struct_string.push_str(&struct_values);
    struct_string.push('}');
    struct_string
}

pub fn vec_to_string<T: Display>(vec: &[T]) -> String {
    let mut vec_string = String::from("&[");
    let mut vec_values = String::new();
    for (i, items) in vec.iter().enumerate() {
        vec_values.push_str(&items.to_string());
        if i != vec.len() - 1 {
            vec_values.push(',');
        }
    }
    vec_string.push_str(&vec_values);
    vec_string.push(']');
    vec_string
}

pub fn option_to_string<T: Display>(option: &Option<T>) -> String {
    if let Some(x) = option {
        x.to_string()
    } else {
        String::from("None")
    }
}

pub fn currency_struct(object: &Value) -> String {
    if let Some(currency) = object.as_object() {
        let mut struct_string = String::from("Currency {");
        let mut struct_values = String::new();
        field_entry!(struct_values, "name", currency);
        field_entry!(struct_values, "code", currency);
        field_entry!(struct_values, "symbol", currency);
        struct_string.push_str(&struct_values);
        struct_string.push('}');
        struct_string
    } else {
        String::from("Default::default()")
    }
}

pub fn timezone_struct(identifier: String) -> String {
    let mut struct_string = String::from("Timezone {");
    let mut struct_values = String::new();
    struct_values.push_str(format!(r#"iana_identifier: {identifier},"#).as_str());
    struct_string.push_str(&struct_values);
    struct_string.push('}');
    struct_string
}

pub fn language_struct(object: &Value) -> String {
    if let Some(languages) = object.as_object() {
        let mut struct_string = String::from("Language {");
        let mut struct_values = String::new();
        field_entry!(struct_values, "iso639_1", languages);
        field_entry!(struct_values, "iso639_2", languages);
        field_entry!(struct_values, "name", languages);
        struct_values.push_str(
            format!(
                r#"native_name: Some({}),"#,
                option_to_string(&languages.get("nativeName"))
            )
            .as_str()
            .replace("null", "None")
            .as_str()
            .replace("Some(None)", "None")
            .as_str(),
        );
        struct_string.push_str(&struct_values);
        struct_string.push('}');
        struct_string
    } else {
        String::from("Default::default()")
    }
}
