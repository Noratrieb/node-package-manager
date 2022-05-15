use indexmap::map::IndexMap;
use node_semver::{Range, Version};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Human {
    pub name: String,
    pub url: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Person {
    Simple(String),
    Expanded(Human),
}

#[derive(Debug, Deserialize)]
pub struct ExpandedFunding {
    pub r#type: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Funding {
    Simple(String),
    Expanded(ExpandedFunding),
    Multiple(Vec<ExpandedFunding>),
}

#[derive(Debug, Deserialize)]
pub struct Bugs {
    pub url: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub r#type: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Bin {
    Single(String),
    Multiple(IndexMap<String, String>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Man {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct PeerDependencyMeta {
    pub optional: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Override {
    Version(String),
    Nested(IndexMap<String, Override>),
}

/// <https://docs.npmjs.com/cli/v8/configuring-npm/package-json>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub name: String,
    pub version: Version,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub homepage: Option<String>,
    pub bugs: Option<Bugs>,
    pub license: Option<String>,
    pub author: Option<Person>,
    pub contributors: Option<Vec<Person>>,
    pub funding: Option<Funding>,
    pub files: Option<Vec<String>>,
    pub main: Option<String>,
    pub browser: Option<String>,
    pub bin: Option<Bin>,
    pub man: Option<Man>,
    pub directories: Option<IndexMap<String, String>>,
    pub repository: Option<Repository>,
    pub scripts: Option<IndexMap<String, String>>,
    pub config: Option<IndexMap<String, serde_json::Value>>,
    pub dependencies: Option<IndexMap<String, Range>>,
    pub dev_dependencies: Option<IndexMap<String, Range>>,
    pub peer_dependencies: Option<IndexMap<String, Range>>,
    pub peer_dependencies_meta: Option<IndexMap<String, PeerDependencyMeta>>,
    pub bundled_dependencies: Option<Vec<String>>,
    pub optional_dependencies: Option<IndexMap<String, Range>>,
    pub overrides: Option<IndexMap<String, Override>>,
    pub engines: Option<IndexMap<String, String>>,
    pub os: Option<Vec<String>>,
    pub cpu: Option<Vec<String>>,
    pub private: Option<bool>,
    pub publish_config: Option<IndexMap<String, serde_json::Value>>,
    pub workspaces: Option<Vec<String>>,
}
