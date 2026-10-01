# Japan administrative areas HAT

Japan-specific HAT for selecting one of the 47 prefectures by its exact
ISO 3166-2 subdivision code. It owns the Japanese and English labels used by
the residence profile and does not infer a prefecture from free-form text,
postal addresses, coordinates, or IP addresses.

`list_for_country("JP")` returns the immutable release list. Every other
country code is unresolved. `select_exact` accepts only an exact released code
and returns `administrative-area-unknown` for every unrecognized value.

The package grants only read access to public administrative-area reference
data. It contains no user data, credential, network endpoint, or authority
grant. Hatter consumes the immutable package artifact without linking this
crate.
