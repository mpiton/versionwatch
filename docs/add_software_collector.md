# Adding a New Software Collector to VersionWatch

This guide explains how to add support for a new software ("collector") in the VersionWatch project. Follow these steps to ensure your contribution is robust, maintainable, and consistent with project standards.

---

## 1. Overview

A "collector" is a Rust struct implementing the `Collector` trait, responsible for fetching version information for a specific software product. Each collector uses intelligent fallback strategies to ensure reliable data collection even when APIs are rate-limited or unavailable.

The `Collector` trait returns a `Vec<ProductCycle>` containing version information with release dates, EOL dates, and LTS status.

---

## 2. Current Architecture

### ProductCycle Structure
```rust
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ProductCycle {
    pub name: String,                              // Version number (e.g., "1.21.0")
    pub release_date: Option<chrono::NaiveDate>,   // Release date if available
    pub eol_date: Option<chrono::NaiveDate>,       // End-of-life date if available
    pub lts: bool,                                 // Whether this is an LTS version
}
```

### Collector Trait
```rust
#[async_trait]
pub trait Collector: Send + Sync {
    fn name(&self) -> &str;
    async fn collect(&self) -> Result<Vec<ProductCycle>, Error>;
}
```

---

## 3. Steps to Add a New Collector

### 3.1. Create the Collector File
- Create a new file: `crates/versionwatch-collect/src/<software>.rs`
- Use an existing collector (e.g., `nginx.rs`, `docker.rs`, `python.rs`) as a template.

### 3.2. Implement the Collector with Fallback Strategy

**Essential Components:**
- **Primary source**: Official API or repository
- **Alternative sources**: Docker Hub, Maven Central, MetaCPAN, GitHub
- **Known versions**: Curated list as final fallback
- **Error handling**: Proper error propagation and logging
- **Deduplication**: Use `HashSet` to avoid duplicate versions

**Example Structure:**
```rust
use crate::{Collector, Error};
use async_trait::async_trait;
use regex::Regex;
use semver::Version;
use std::collections::HashSet;
use versionwatch_core::domain::product_cycle::ProductCycle;

pub struct MySoftwareCollector {
    name: String,
    github_token: Option<String>,
}

impl MySoftwareCollector {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            github_token: std::env::var("GITHUB_TOKEN").ok(),
        }
    }

    async fn fetch_primary_source(&self) -> Result<Vec<ProductCycle>, Error> {
        // Implement primary API call
    }

    async fn fetch_alternative_source(&self) -> Result<Vec<ProductCycle>, Error> {
        // Implement fallback API call (Docker Hub, Maven Central, etc.)
    }

    fn get_known_versions(&self) -> Vec<ProductCycle> {
        // Return curated list of stable versions as final fallback
        vec![
            ProductCycle {
                name: "1.0.0".to_string(),
                release_date: None,
                eol_date: None,
                lts: false,
            },
            // ... more versions
        ]
    }
}

#[async_trait]
impl Collector for MySoftwareCollector {
    fn name(&self) -> &str {
        &self.name
    }

    async fn collect(&self) -> Result<Vec<ProductCycle>, Error> {
        // Try primary source first
        if let Ok(cycles) = self.fetch_primary_source().await {
            if !cycles.is_empty() {
                return Ok(cycles);
            }
        }

        // Fallback to alternative source
        if let Ok(cycles) = self.fetch_alternative_source().await {
            if !cycles.is_empty() {
                return Ok(cycles);
            }
        }

        // Final fallback to known versions
        Ok(self.get_known_versions())
    }
}
```

### 3.3. Register the Collector

**In `crates/versionwatch-collect/src/lib.rs`:**
```rust
pub mod my_software;  // Add this line

// At the bottom:
pub use my_software::MySoftwareCollector;
```

**In `crates/versionwatch-cli/src/main.rs`:**
```rust
// Add import at the top:
use versionwatch_collect::{
    // ... existing imports
    my_software::MySoftwareCollector,
};

// Add case in get_collector function:
fn get_collector(
    target: &ConfigTarget,
    github_token: Option<&str>,
) -> Option<Box<dyn Collector + Send + Sync>> {
    // ... existing cases
    } else if target.name == "my-software" {
        Some(Box::new(MySoftwareCollector::new(&target.name)))
    } else {
        None
    }
}
```

### 3.4. Update Configuration
**In `config/base.yml`:**
  ```yaml
targets:
  # ... existing targets
  - name: "my-software"
    enabled: true
  ```

### 3.5. Test Your Collector
```bash
# Test with dry run
cargo run -- --dry-run

# Check for your collector in output
# Should show: "Collected X versions for my-software"

# Run quality checks
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

---

## 4. Best Practices & Implementation Patterns

### 4.1. Resilience Strategies

**Multi-Source Fallback:**
```rust
async fn collect(&self) -> Result<Vec<ProductCycle>, Error> {
    // 1. Try official API
    if let Ok(cycles) = self.fetch_official_api().await {
        if !cycles.is_empty() {
            return Ok(cycles);
        }
    }

    // 2. Try Docker Hub
    if let Ok(cycles) = self.fetch_docker_hub().await {
        if !cycles.is_empty() {
            return Ok(cycles);
        }
    }

    // 3. Try GitHub releases
    if let Ok(cycles) = self.fetch_github_releases().await {
        if !cycles.is_empty() {
            return Ok(cycles);
        }
    }

    // 4. Final fallback to known versions
    Ok(self.get_known_versions())
}
```

**Version Deduplication:**
```rust
let mut seen_versions = HashSet::new();
let cycles: Vec<ProductCycle> = raw_versions
    .into_iter()
    .filter_map(|version_str| {
        if let Ok(version) = Version::parse(&version_str) {
            if seen_versions.contains(&version) {
                return None; // Skip duplicates
            }
            seen_versions.insert(version.clone());
            
            Some(ProductCycle {
                name: version.to_string(),
                release_date: None,
                eol_date: None,
                lts: false,
            })
        } else {
            None
        }
    })
    .collect();
```

**Error Handling:**
```rust
async fn fetch_api(&self) -> Result<Vec<ProductCycle>, Error> {
    let response = client
        .get(url)
        .header("User-Agent", "versionwatch-collector")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(Error::Other(anyhow::anyhow!(
            "API returned status: {}",
            response.status()
        )));
    }

    let data: ApiResponse = response.json().await?;
    // Process data...
}
```

### 4.2. Common Data Sources

**Docker Hub API:**
```rust
async fn fetch_docker_hub_versions(&self) -> Result<Vec<ProductCycle>, Error> {
    let url = "https://hub.docker.com/v2/repositories/library/my-software/tags/?page_size=500";
    // Implementation...
}
```

**GitHub Releases:**
```rust
async fn fetch_github_releases(&self) -> Result<Vec<ProductCycle>, Error> {
    let url = "https://api.github.com/repos/owner/repo/releases?per_page=100";
    // Add Authorization header if github_token is available
}
```

**Maven Central:**
```rust
async fn fetch_maven_versions(&self) -> Result<Vec<ProductCycle>, Error> {
    let url = "https://search.maven.org/solrsearch/select?q=g:group+AND+a:artifact&rows=100&wt=json";
    // Implementation...
}
```

### 4.3. Version Parsing

**Regex Patterns:**
```rust
// Standard semantic versions
let re = Regex::new(r"^(\d+\.\d+\.\d+)(?:-[a-zA-Z0-9]+)?$").unwrap();

// Docker tag format
let re = Regex::new(r"^(\d+\.\d+(?:\.\d+)?)(?:-[a-zA-Z0-9]+)?$").unwrap();

// GitHub release tags
let re = Regex::new(r"^v?(\d+\.\d+\.\d+)$").unwrap();
```

**Version Normalization:**
```rust
// Add .0 if needed for semver compatibility
let normalized_version = if version_str.matches('.').count() == 1 {
    format!("{version_str}.0")
} else {
    version_str.to_string()
};
```

---

## 5. Complete Example: Adding "ExampleSoft"

**File: `crates/versionwatch-collect/src/example_soft.rs`**
   ```rust
use crate::{Collector, Error};
   use async_trait::async_trait;
use regex::Regex;
use semver::Version;
use std::collections::HashSet;
use versionwatch_core::domain::product_cycle::ProductCycle;

#[derive(serde::Deserialize, Debug)]
struct DockerHubResponse {
    results: Vec<DockerHubTag>,
}

#[derive(serde::Deserialize, Debug)]
struct DockerHubTag {
    name: String,
}

pub struct ExampleSoftCollector {
    name: String,
}

impl ExampleSoftCollector {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    async fn fetch_docker_hub_versions(&self) -> Result<Vec<ProductCycle>, Error> {
        let url = "https://hub.docker.com/v2/repositories/library/example-soft/tags/?page_size=100";
        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .header("User-Agent", "versionwatch-collector")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::Other(anyhow::anyhow!(
                "Docker Hub API returned status: {}",
                response.status()
            )));
        }

        let data: DockerHubResponse = response.json().await?;
        let re = Regex::new(r"^(\d+\.\d+\.\d+)$").unwrap();
        let mut seen_versions = HashSet::new();

        let cycles: Vec<ProductCycle> = data
            .results
            .into_iter()
            .filter_map(|tag| {
                if let Some(captures) = re.captures(&tag.name) {
                    let version_str = captures.get(1)?.as_str();
                    if let Ok(version) = Version::parse(version_str) {
                        if seen_versions.contains(&version) {
                            return None;
                        }
                        seen_versions.insert(version.clone());

                        Some(ProductCycle {
                            name: version.to_string(),
                            release_date: None,
                            eol_date: None,
                            lts: false,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        Ok(cycles)
    }

    fn get_known_versions(&self) -> Vec<ProductCycle> {
        vec![
            ProductCycle {
                name: "2.1.0".to_string(),
                release_date: None,
                eol_date: None,
                lts: true,
            },
            ProductCycle {
                name: "2.0.5".to_string(),
                release_date: None,
                eol_date: None,
                lts: false,
            },
            ProductCycle {
                name: "1.9.8".to_string(),
                release_date: None,
                eol_date: None,
                lts: true,
            },
        ]
    }
}

   #[async_trait]
   impl Collector for ExampleSoftCollector {
    fn name(&self) -> &str {
        &self.name
    }

    async fn collect(&self) -> Result<Vec<ProductCycle>, Error> {
        // Try Docker Hub first
        if let Ok(cycles) = self.fetch_docker_hub_versions().await {
            if !cycles.is_empty() {
                return Ok(cycles);
            }
        }

        // Fallback to known versions
        Ok(self.get_known_versions())
       }
   }
   ```

---

## 6. Quality Checklist

Before submitting your collector:

- [ ] **Implements fallback strategy** (multiple data sources)
- [ ] **Handles rate limiting** gracefully
- [ ] **Deduplicates versions** using HashSet
- [ ] **Uses proper error handling** with Error enum
- [ ] **Includes known versions** as final fallback
- [ ] **Parses versions correctly** with regex
- [ ] **Passes cargo fmt** (code formatting)
- [ ] **Passes cargo clippy** (no warnings)
- [ ] **Passes cargo test** (all tests)
- [ ] **Works in dry-run mode** (`cargo run -- --dry-run`)
- [ ] **Registered in lib.rs and main.rs**
- [ ] **Added to config/base.yml**
- [ ] **Documented any API quirks** in code comments

---

## 7. Useful Resources

- **Collector trait**: `crates/versionwatch-collect/src/lib.rs`
- **ProductCycle struct**: `crates/versionwatch-core/src/domain/product_cycle.rs`
- **Example collectors**: `crates/versionwatch-collect/src/nginx.rs`, `crates/versionwatch-collect/src/docker.rs`
- **CLI registration**: `crates/versionwatch-cli/src/main.rs`
- **Configuration**: `config/base.yml`

---

## 8. Common APIs and Patterns

### Docker Hub API
- **URL**: `https://hub.docker.com/v2/repositories/library/{software}/tags/`
- **Rate limits**: Generally permissive
- **Best for**: Official container images

### GitHub Releases API
- **URL**: `https://api.github.com/repos/{owner}/{repo}/releases`
- **Rate limits**: 60/hour without token, 5000/hour with token
- **Best for**: Official software releases

### Maven Central API
- **URL**: `https://search.maven.org/solrsearch/select?q=g:{group}+AND+a:{artifact}`
- **Rate limits**: Generally permissive
- **Best for**: Java/Scala/Kotlin libraries

### Official APIs
Many projects provide their own APIs:
- Node.js: `https://nodejs.org/dist/index.json`
- Go: `https://go.dev/dl/?mode=json`
- PHP: `https://www.php.net/releases/index.php?json=1`

---

Thank you for contributing to VersionWatch! Your collector will help track software versions with industry-leading reliability. 🚀 