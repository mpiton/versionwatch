# Adding a New Software Collector to VersionWatch

This guide explains how to add support for a new software ("collector") in the VersionWatch project. Follow these steps to ensure your contribution is robust, maintainable, and consistent with project standards.

---

## 1. Overview

A "collector" is a Rust struct implementing the `Collector` trait, responsible for fetching the latest version (and optionally LTS/EOL info) for a given software. Each collector is a separate file in `crates/versionwatch-collect/src/`.

---

## 2. Steps to Add a New Collector

### 2.1. Create the Collector File
- Create a new file: `crates/versionwatch-collect/src/<software>.rs`
- Use an existing collector (e.g., `nginx.rs`, `docker.rs`, `python.rs`) as a template.

### 2.2. Implement the Collector
- Define a struct (e.g., `MySoftwareCollector`).
- Implement the `Collector` trait for your struct.
- Fetch data from the official API, website, or GitHub releases/tags.
- Parse and normalize the version string(s).
- Fill all required DataFrame columns:
  - `name`, `current_version`, `latest_version`, `latest_lts_version`, `is_lts`, `eol_date`, `release_notes_url`, `cve_count`
- Handle errors using the `Error` enum from `lib.rs`.
- Handle API rate limiting and edge cases explicitly.

### 2.3. Register the Collector
- In `crates/versionwatch-collect/src/lib.rs`:
  - Add `pub mod <software>;` at the top.
  - Add `pub use <software>::<MySoftwareCollector>;` at the bottom.
- In `crates/versionwatch-cli/src/main.rs`:
  - Import your collector.
  - Add a match arm in the collector registration (see other collectors for pattern).

### 2.4. Update the Configuration
- In `config/base.yml`, add an entry:
  ```yaml
  - name: "<software>"
    enabled: true
  ```
- The `name` must match the collector's `fn name()` return value.

### 2.5. Test Your Collector
- Run `cargo run --bin versionwatch-cli` and verify your software appears in the output.
- Check for correct version, release notes, and error handling.
- Run `cargo clippy --all-targets --all-features -- -D warnings` and `cargo fmt`.
- Add or update tests if needed (see other collectors for test patterns).

### 2.6. Document and Commit
- Document any API quirks or parsing logic in code comments.
- Update the main `README.md` to mention your new collector (see below).
- Commit your changes with a clear message.

---

## 3. Example: Adding "example-soft"

1. Create `crates/versionwatch-collect/src/example_soft.rs`:
   ```rust
   use super::{Collector, Error};
   use async_trait::async_trait;
   use polars::prelude::*;

   pub struct ExampleSoftCollector;

   #[async_trait]
   impl Collector for ExampleSoftCollector {
       fn name(&self) -> &'static str { "example-soft" }
       async fn collect(&self) -> Result<DataFrame, Error> {
           // Fetch and parse version info here
           let df = df!(
               "name" => &["example-soft"],
               "current_version" => &[None::<String>],
               "latest_version" => &["1.2.3"],
               "latest_lts_version" => &[None::<String>],
               "is_lts" => &[false],
               "eol_date" => &[None::<i64>],
               "release_notes_url" => &[Some("https://example.com/release".to_string())],
               "cve_count" => &[0_i32],
           )?;
           Ok(df)
       }
   }
   ```
2. Register in `lib.rs` and `main.rs` as described above.
3. Add to `config/base.yml`:
   ```yaml
   - name: "example-soft"
     enabled: true
   ```
4. Test and document.

---

## 4. Best Practices & Checklist

- [ ] Use clear, robust error handling (no panics)
- [ ] Handle API rate limiting and edge cases
- [ ] Use the same DataFrame schema as other collectors
- [ ] Add code comments for non-obvious logic
- [ ] Run `cargo fmt` and `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Test manually and/or add automated tests
- [ ] Update documentation (this file and README)
- [ ] Use a clear, conventional commit message

---

## 5. Useful Links
- [Collector trait definition](../crates/versionwatch-collect/src/lib.rs)
- [Example collector: nginx.rs](../crates/versionwatch-collect/src/nginx.rs)
- [Configuration file](../config/base.yml)
- [CLI main file](../crates/versionwatch-cli/src/main.rs)

---

## 6. Updating the README

After adding your collector, update the `README.md` to:
- List your software in the "Supported Software" section
- Optionally, add a note about the data source/API used

---

Thank you for contributing to VersionWatch! 