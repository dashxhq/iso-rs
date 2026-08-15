# AGENTS.md

Guidance for AI coding agents working in this repository.

## What this crate is

`iso-rs` is a zero-runtime-cost lookup crate for ISO country data (ISO 3166-1 country
codes, ISO 639-1/2 language codes, ISO 4217 currencies, IANA timezones, capitals,
regions, calling codes).

There is no runtime parsing and no network access. Two vendored JSON snapshots are
compiled into `phf` perfect-hash maps by a build script; the library is a thin set of
`.get()` wrappers over those maps. Published on crates.io as `iso-rs`.

## Commands

```bash
cargo test                          # unit tests + doctests
cargo test --no-default-features    # MUST pass — CI runs this
cargo build
cargo fmt
cargo clippy --all-targets
cargo doc --open
```

Touching anything under `build/` forces a full rebuild and regenerates a ~760 KB
`codegen.rs`. Expect build-script work to be the slow part of any edit there.

## Layout

| Path | Role |
|---|---|
| `src/lib.rs` | The entire public API — structs, query methods, tests. ~250 lines. |
| `build/build.rs` | Build-script crate root. Declares `codegen`, `countries`, `macros`, `time`. |
| `build/countries/mod.rs` | Main codegen driver: JSON → `CountryData` → phf maps. |
| `build/countries/country_data.rs` | Builder for the intermediate `CountryData` record. |
| `build/codegen/mod.rs` | Emits Rust struct literals as **strings**. |
| `build/codegen/map_builder.rs` | Wraps the five `phf_codegen::Map`s. |
| `build/macros.rs` | `value_or_none!`, `vec_or_none!`, `field_entry!`, `tokens!`, etc. |
| `build/countries.json` | Vendored [restcountries](https://gitlab.com/amatos/rest-countries) snapshot. |
| `build/timezones.json` | Vendored timezonedb snapshot, keyed by `countryCode`. |

`build/` is a separate compilation unit from `src/`. They share no code; `build/` items
are reachable via `crate::` only from within the build script.

## Pipeline

```
countries.json ─┐
                ├─> CountryData (all fields as String) ─> country_struct() ─> "Country { ... }"
timezones.json ─┘                                                                   │
                                                                                    v
                                       phf_codegen::Map  ─>  TokenStream  ─>  $OUT_DIR/codegen.rs
                                                                                    │
                                                        src/lib.rs: include!(...) ───┘
```

The generated file defines `NAMES`, `CAPTIAL`, `REGIONS`, `ALPHA_2`, `ALPHA_3`.

## Feature flags

`from_capitals`, `from_alpha_2`, `from_alpha_3`, `from_regions` — all on by default via
the `all` feature. Each gates *both* the generated static (in `countries/mod.rs`'s
`quote!` block) and the query method (in `src/lib.rs`).

Adding a query method means adding `#[cfg(feature = "...")]` in **three** places: the
static, the method, and the test. Then verify with `cargo test --no-default-features`.

## Tests

All tests live in `src/lib.rs`. Two kinds, both real:

- The `india_check!` macro asserts the full field set for India. Reuse it for new
  lookup methods rather than writing fresh assertions.
- Doctests on every public method. These are the examples users read — keep them
  accurate and runnable.

## CI

`.github/workflows/rust.yml` on push/PR to `main`: `cargo build`, `cargo test`,
`cargo test --no-default-features`. No `fmt` or `clippy` gate.

Current baseline, so you can tell your changes apart from pre-existing noise:

- `cargo fmt --check` reports diffs in `build/countries/mod.rs` and `build/time.rs`
  (edition-2024 import ordering).
- `cargo clippy` reports 3 `collapsible_if` warnings in the build script.

Fix these only if that is the task. Do not fold an unrelated repo-wide reformat into a
behavioral change — it makes the diff unreviewable.

## Code style

Standard `rustfmt`, edition 2024. Match the surrounding code's idiom.

`#![forbid(unsafe_code)]` — no exceptions. `Cargo.lock` is gitignored; don't commit it.

### Comments

Keep comments short and rare. This is a strict convention in this repo.

- Comment only what the code cannot say: a non-obvious constraint, a load-bearing
  workaround, or a "why" that would otherwise be lost.
- One or two lines. If a comment needs a paragraph, the code likely needs a better name
  or a smaller function.
- Never narrate. `// increment the counter`, `// loop over countries`, section banners,
  and restatements of the signature are all noise — delete them.
- No changelog comments. Don't write `// changed from X`, `// was previously`, or
  `// added by request` — that is what git history is for.
- Doc comments (`///`) on public items are the exception: they are the API contract and
  should stay complete, with a runnable example where useful.

Prefer deleting a stale comment over updating it if the code now reads clearly on its own.
