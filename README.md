# Sqruff Plugin

[![ci](https://github.com/fluentci-io/sqruff-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/sqruff-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with [sqruff](https://github.com/quarylabs/sqruff) installed.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm sqruff setup
```

## Functions

| Name   | Description         |
| ------ | ------------------- |
| setup  | Installs sqruff.    |
| lint   | Lint files          |
| fix    | Fix files           |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/sqruff@v0.1.1?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: sqruff
    args: |
      setup
- name: Show sqruff version
  run: |
    type sqruff
    sqruff --version
```
