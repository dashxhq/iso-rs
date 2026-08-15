#!/usr/bin/env bash
#
# Refresh build/countries.json.
#
# The hosted restcountries API the build script originally fetched from is gone:
# /v1 through /v4 were taken down, and /v5 needs an account and an API key. The
# data still lives in the API's own repository, in the same v2 schema that
# build/countries/mod.rs reads, so that is what we pull.

set -euo pipefail

SOURCE_URL="${COUNTRIES_URL:-https://gitlab.com/amatos/rest-countries/-/raw/master/src/main/resources/countriesV2.json}"

root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
target="$root/build/countries.json"

for dep in curl jq; do
  command -v "$dep" >/dev/null || { echo "error: $dep is required" >&2; exit 1; }
done

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

echo "fetching $SOURCE_URL"
curl -fsSL --retry 3 --max-time 120 "$SOURCE_URL" -o "$tmp/raw.json"

# jq . normalises formatting only -- key order and values are left alone -- so
# future refreshes diff on content rather than on upstream whitespace.
jq . "$tmp/raw.json" >"$tmp/countries.json"

check() {
  local message="$1" filter="$2"
  local offenders
  offenders="$(jq -r "$filter" "$tmp/countries.json")"
  if [ -n "$offenders" ]; then
    echo "error: $message" >&2
    printf '%s\n' "$offenders" | head -20 | sed 's/^/  /' >&2
    exit 1
  fi
}

jq -e 'type == "array" and length >= 200' "$tmp/countries.json" >/dev/null \
  || { echo "error: expected an array of at least 200 countries" >&2; exit 1; }

# These three key the generated phf maps; a country missing one is unreachable.
check "records missing name/alpha2Code/alpha3Code" '
  to_entries[]
  | select((.value.name // "") == "" or (.value.alpha2Code // "") == "" or (.value.alpha3Code // "") == "")
  | "record \(.key)"'

# get_countries() drops any country whose name it has already seen, so a repeated
# or mislabelled name silently removes a country from every map, including the
# ones keyed by something else. This is what cost us Hong Kong.
check "duplicate country names (each would drop a country from the build)" '
  group_by(.name)[] | select(length > 1) | "\(.[0].name) x\(length)"'

# codegen emits Rust source by string formatting, and field_entry! rewrites the
# substring "null" to "None" across the whole rendered field, data included.
check 'values containing "null" (would be corrupted into "None" by field_entry!)' '
  .[] as $c
  | $c | {name, capital, region, alpha2Code, alpha3Code, callingCodes, currencies, languages}
  | [.. | strings] | .[]
  | select(contains("null"))
  | "\($c.alpha2Code): \(.)"'

# A control character reaches the generated source verbatim and will not compile.
check "values containing control characters (would break the generated source)" '
  .[] as $c
  | $c | {name, capital, region, alpha2Code, alpha3Code, callingCodes, currencies, languages}
  | [.. | strings] | .[]
  | select(test("[[:cntrl:]]"))
  | "\($c.alpha2Code): \(@json)"'

if [ -f "$target" ]; then
  before="$(jq -r '[.[].alpha2Code] | sort | .[]' "$target")"
  after="$(jq -r '[.[].alpha2Code] | sort | .[]' "$tmp/countries.json")"
  added="$(comm -13 <(echo "$before") <(echo "$after") | tr '\n' ' ')"
  removed="$(comm -23 <(echo "$before") <(echo "$after") | tr '\n' ' ')"
  echo "countries: $(jq length "$target") -> $(jq length "$tmp/countries.json")"
  [ -n "${added// }" ] && echo "  added:   $added"
  [ -n "${removed// }" ] && echo "  removed: $removed"
fi

mv "$tmp/countries.json" "$target"
echo "wrote $target"
echo "run 'cargo test' to regenerate and verify"
