# Using hat-japan-administrative-areas

Select one of Japan’s 47 prefectures using an exact released subdivision code.

## Before you start

Selection is explicit. Free-form text, addresses, coordinates and IP addresses are not used to infer a prefecture.

## First steps

Run from the repository root:

```sh
cargo test --locked
```

## How to assess the result

- List Japanese and English prefecture labels.
- Reject unknown codes without guessing a residence.

A passing source-level check establishes only what that check observes. Keep missing configuration, unavailable services and unverified deployment paths visible.

## Continue reading

[Repository overview](../README.md)
