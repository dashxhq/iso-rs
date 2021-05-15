# iso-rs
[![MIT license](https://img.shields.io/badge/License-MIT-blue.svg)](https://lbesson.mit-license.org/)

ISO-rs provides methods to query ISO country data. This includes

- names
- capitals
- regions
- alpha 2 codes
- alpha 3 codes
- timezones
- currencies
- languages
- call codes

# Overview

```rust
use iso_rs::prelude::*;

let country = Country::from_name("India").unwrap();
assert_eq!(country.capital.unwrap(), "New Delhi");
```

# LICENSE 
MIT
