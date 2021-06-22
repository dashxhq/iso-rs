// Map through values which are supposed to be
// in the form of arrays, if not then just replace it
// with None
#[macro_export]
macro_rules! vec_or_none {
    // Just convert the json array into string counterparts
    ( $key : expr, $country_data : expr ) => {
        $country_data
            .get($key)
            .map(|value| {
                if let Some(x) = value.as_array() {
                    x.iter().map(|k| k.to_string()).collect::<Vec<_>>()
                } else {
                    vec!["None"]
                        .iter()
                        .map(|k| k.to_string())
                        .collect::<Vec<_>>()
                }
            })
            .unwrap_or(Vec::new());
    };
    // for generating currencies structs, convert the currencies json
    // object to a valid rust struct
    ( $key : expr, $country_data : expr, currencies ) => {
        $country_data
            .get($key)
            .map(|value| {
                if let Some(x) = value.as_array() {
                    x.iter().map(|k| currency_struct(k)).collect::<Vec<_>>()
                } else {
                    vec!["None"]
                        .iter()
                        .map(|k| k.to_string())
                        .collect::<Vec<_>>()
                }
            })
            .unwrap_or(Vec::new());
    };
    // for generating language structs, convert the language json
    // object to a valid rust struct
    ( $key : expr, $country_data : expr, languages ) => {
        $country_data
            .get($key)
            .map(|value| {
                if let Some(x) = value.as_array() {
                    x.iter().map(|k| language_struct(k)).collect::<Vec<_>>()
                } else {
                    vec!["None"]
                        .iter()
                        .map(|k| k.to_string())
                        .collect::<Vec<_>>()
                }
            })
            .unwrap_or(Vec::new());
    };
    // for generating timezone structs, convert the timezone json
    // object to a valid rust struct
    ( $key : expr, $country_data : expr, $timezone : expr ) => {
        $country_data
            .get($key)
            .map(|value| {
                if let Some(x) = value.as_array() {
                    x.iter()
                        .map(|k| timezone_struct(k, $timezone))
                        .collect::<Vec<_>>()
                } else {
                    vec!["None"]
                        .iter()
                        .map(|k| k.to_string())
                        .collect::<Vec<_>>()
                }
            })
            .unwrap_or(Vec::new());
    };
}

// Insert a value and convert it to String or insert a "None" String
#[macro_export]
macro_rules! value_or_none {
    ( $key : expr, $country_data : expr ) => {
        $country_data
            .get($key)
            .map(|value| value.to_string())
            .unwrap_or(String::from("None"));
    };
}

// parse the token stream from the built static map initilization
#[macro_export]
macro_rules! tokens {
    ( $variable : expr  ) => {
        TokenStream::from_str(&$variable.build().to_string())?;
    };
}

// Entry fields, purify Some(None) => None
// and null => None
#[macro_export]
macro_rules! field_entry {
    ( $struct : expr, $string : expr, $index : expr) => {
        $struct.push_str(
            format!(
                r#"{}: Some({}),"#,
                $string,
                option_to_string(&$index.get($string))
            )
            .as_str()
            .replace("null", "None")
            .as_str()
            .replace("Some(None)", "None")
            .as_str()
            .replace("(none)", "None")
            .as_str(),
        );
    };

    ( $struct : expr, $name : tt) => {
        $struct.push_str(format!(r#"{}: {},"#, stringify!($name), $name).as_str());
    };

    ( $struct : expr, Some($name : tt)) => {
        $struct.push_str(
            format!(r#"{}: Some({}),"#, stringify!($name), $name)
                .as_str()
                .replace("null", "None")
                .as_str()
                .replace("Some(None)", "None")
                .as_str(),
        );
    };
}

// macro to quickly generate methods with relations to fields
#[macro_export]
macro_rules! map_method {
    ( $function_name : ident ) => {
        paste::item! {
            pub fn [ < $function_name > ] ( &mut self ) -> &mut Map<&'a str> {
                &mut self.$function_name
            }
        }
    };
}

// macro to quickly generate methods with relations to fields
#[macro_export]
macro_rules! hash_map_to_static {
    ( $hash_map : expr, $static_map : expr, $item : ident ) => {
        $hash_map.iter().for_each(|$item| {
            $static_map.$item().entry($item.0, &vec_to_string($item.1));
        });
    };
}
