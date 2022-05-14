#![allow(dead_code)]

use color_eyre::Result;
use indexmap::IndexMap;
use reqwest::blocking::Client;
use serde::Deserialize;
use tracing::debug;

use crate::{
    manifest::{Bugs, Human, Person, Repository},
    PackageJson,
};

#[derive(Debug, Deserialize)]
pub struct Dist {
    pub shasum: String,
    pub tarball: String,
    pub integrity: Option<String>,
    #[serde(rename = "fileCount")]
    pub file_count: Option<u32>,
    #[serde(rename = "unpackedSize")]
    pub unpacked_size: Option<u32>,
    #[serde(rename = "npm-signature")]
    pub npm_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VersionMeta {
    pub _from: Option<String>,
    pub _id: String,
    #[serde(rename = "_nodeVersion")]
    pub _node_version: String,
    #[serde(rename = "_npmUser")]
    pub _npm_user: Person,
    #[serde(rename = "_npmVersion")]
    pub _npm_version: String,
    pub _shasum: Option<String>,
    #[serde(rename = "_hasShrinkwrap")]
    pub _has_shrinkwrap: Option<bool>,
    pub dist: Dist,
    pub files: Vec<String>,

    #[serde(flatten)]
    pub package_json: PackageJson,
}

#[derive(Debug, Deserialize)]
pub struct PackageMeta {
    pub _id: String,
    pub _rev: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: IndexMap<String, String>,
    pub name: String,
    pub time: IndexMap<String, String>,
    pub users: IndexMap<String, bool>,
    pub versions: IndexMap<String, VersionMeta>,

    pub author: Human,
    pub bugs: Option<Bugs>,
    pub contributors: Option<Vec<Human>>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub license: Option<String>,
    pub maintainers: Option<Vec<Human>>,
    pub readme: Option<String>,
    #[serde(rename = "readmeFilename")]
    pub readme_filename: Option<String>,
    pub repository: Option<Repository>,
}

pub struct NpmClient {
    reqwest: Client,
}

const BASE_URL: &str = "https://registry.npmjs.org";

impl NpmClient {
    pub fn new() -> Self {
        let reqwest = Client::new();
        Self { reqwest }
    }

    #[tracing::instrument(skip(self))]
    pub fn inspect_package(&self, name: &str) -> Result<PackageMeta> {
        let res = self.reqwest.get(format!("{BASE_URL}/{name}")).send()?;
        let code = res.status();
        let body = res.text()?;
        let meta = serde_json::from_str::<PackageMeta>(&body)?;

        debug!(?code, ?meta, "Received response");
        Ok(meta)
    }
}
