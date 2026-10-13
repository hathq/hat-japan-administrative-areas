# hat-japan-administrative-areas

公開された都道府県コードから、日本の47都道府県を明示的に選択できます。住所やIPから居住地を推測しません。

## 利用前の確認

実装済みの範囲、必要な依存関係、検証コマンドを以下の英語説明に併記しています。操作・配備・公開は、それぞれの権限と設定を確認してから実施してください。

## 使い方

リポジトリ内のサンプル・スキーマ・実装を確認し、用途に必要な入力を明示して利用します。下記のGetting startedに、現行設定に対応する検証コマンドを示しています。

検証結果は実行した範囲だけを示します。未実装の機能、未設定の接続、配備環境の確認を合格扱いにしないでください。

## English

Select one of Japan’s 47 prefectures using an exact released subdivision code.

## What you can do

- List Japanese and English prefecture labels.
- Reject unknown codes without guessing a residence.

## Current scope

Selection is explicit. Free-form text, addresses, coordinates and IP addresses are not used to infer a prefecture.

Package distribution is not activated by this documentation. Use the checked-in source and the declared dependency versions; published availability must be verified separately.

## Getting started

Install Rust 1.97 or newer and make the declared dependencies available. Resolve declared third-party dependencies from public registries and source-only managed components from their fixed public Git revision. No private registry or adjacent checkout is part of the supported OSS source path. Run from this repository:

```sh
cargo test --locked
```

## Documentation and source

[Usage guide](docs/getting-started.md)

[Schemas](schemas) · [Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Generic data dependency

The subdivision codes and names resolve from `zixcel-japan-administrative-areas`
`0.1.0` on crates.io. This product retains the HAT invocation, explicit country
selection and unresolved-input behavior; it no longer duplicates the dataset.

[OSS responsibility and source completion](docs/oss-source-contract.md) defines the supported package boundary and required acceptance. It does not declare unexecuted source, distribution or production checks complete.
