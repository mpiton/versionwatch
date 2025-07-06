#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ProductCycle {
    pub name: String,
    pub release_date: Option<chrono::NaiveDate>,
    pub eol_date: Option<chrono::NaiveDate>,
    pub lts: bool,
}
