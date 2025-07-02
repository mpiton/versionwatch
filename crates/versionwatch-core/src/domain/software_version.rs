use serde::{Deserialize, Serialize};
use time::Date;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SoftwareVersion {
    pub name: String,
    pub current_version: String,
    pub latest_version: String,
    pub latest_lts_version: Option<String>,
    pub is_lts: bool,
    pub eol_date: Option<Date>,
    pub release_notes_url: Option<String>,
    pub cve_count: i32,
}
