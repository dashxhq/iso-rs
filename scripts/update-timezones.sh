#!/usr/bin/env bash
#
# Refresh build/timezones.json from the IANA tz database.
#
# The vendored snapshot came from timezonedb's list-time-zone endpoint, via a
# hardcoded free-tier API key. That response is a repackaging of IANA's
# zone.tab -- diffing the two gives identical country/zone pairs -- so we read
# zone.tab directly: no key, public domain, and the same upstream chrono-tz is
# generated from, which keeps the two from drifting apart.

set -euo pipefail

SOURCE_URL="${TZDATA_URL:-https://data.iana.org/time-zones/tzdata-latest.tar.gz}"

root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
target="$root/build/timezones.json"

for dep in curl jq tar; do
  command -v "$dep" >/dev/null || { echo "error: $dep is required" >&2; exit 1; }
done

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

echo "fetching $SOURCE_URL"
curl -fsSL --retry 3 --max-time 120 "$SOURCE_URL" -o "$tmp/tzdata.tar.gz"
tar -xzf "$tmp/tzdata.tar.gz" -C "$tmp" version zone.tab

version="$(cat "$tmp/version")"
echo "tzdata $version"

# zone.tab is: country code, coordinates, TZ identifier, optional comments.
# Sorted by zone name within each country to match the order the previous
# snapshot produced, so Country::timezones keeps its existing ordering.
jq -R -s --arg version "$version" --arg source "$SOURCE_URL" '
  split("\n")
  | map(select(length > 0 and (startswith("#") | not)) | split("\t"))
  | map(select(length >= 3) | {countryCode: .[0], zoneName: .[2]})
  | sort_by(.countryCode, .zoneName)
  | {version: $version, source: $source, zones: .}
' "$tmp/zone.tab" >"$tmp/timezones.json"

check() {
  local message="$1" filter="$2"
  local offenders
  offenders="$(jq -r "$filter" "$tmp/timezones.json")"
  if [ -n "$offenders" ]; then
    echo "error: $message" >&2
    printf '%s\n' "$offenders" | head -20 | sed 's/^/  /' >&2
    exit 1
  fi
}

jq -e '.zones | length >= 400' "$tmp/timezones.json" >/dev/null \
  || { echo "error: expected at least 400 zones, parsed $(jq '.zones | length' "$tmp/timezones.json")" >&2; exit 1; }

check "malformed rows" '
  .zones[]
  | select((.countryCode | test("^[A-Z]{2}$") | not) or (.zoneName | test("^[A-Za-z0-9_+-]+(/[A-Za-z0-9_+-]+)*$") | not))
  | "\(.countryCode) \(.zoneName)"'

check "duplicate country/zone pairs" '
  .zones | group_by(.countryCode + " " + .zoneName)[] | select(length > 1) | "\(.[0].countryCode) \(.[0].zoneName)"'

if [ -f "$target" ]; then
  before="$(jq -r '[.zones[].zoneName] | unique | .[]' "$target")"
  after="$(jq -r '[.zones[].zoneName] | unique | .[]' "$tmp/timezones.json")"
  added="$(comm -13 <(echo "$before") <(echo "$after") | tr '\n' ' ')"
  removed="$(comm -23 <(echo "$before") <(echo "$after") | tr '\n' ' ')"
  echo "zones: $(jq '.zones | length' "$target") -> $(jq '.zones | length' "$tmp/timezones.json")"
  [ -n "${added// }" ] && echo "  added:   $added"
  [ -n "${removed// }" ] && echo "  removed: $removed"
fi

mv "$tmp/timezones.json" "$target"
echo "wrote $target"
echo "run 'cargo test' to regenerate and verify"
