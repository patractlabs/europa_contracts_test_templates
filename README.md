# europa_contracts_test_templates
Provide some templates for test the contract in europa.

## Build

### precondition

You need to use our modified `cargo contract`.

Optional: Install it use the following command:

```
cargo install cargo-contract --git https://github.com/patractlabs/cargo-contract -f --branch=cmd/debug-flag
```

### build contract

```
cargo +nightly contract build --debug
```

## Test

Note: `put code` use the `xxx.src.wasm`.

