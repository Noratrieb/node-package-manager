use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub private: Option<bool>,
    #[serde(default = "IndexMap::new")]
    pub scripts: IndexMap<String, String>,
    #[serde(default = "IndexMap::new")]
    pub dependencies: IndexMap<String, String>,
    #[serde(default = "IndexMap::new")]
    pub dev_dependencies: IndexMap<String, String>,
}
