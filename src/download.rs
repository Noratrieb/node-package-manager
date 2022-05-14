#![allow(dead_code)]

use color_eyre::Result;
use indexmap::IndexMap;
use reqwest::blocking::Client;
use serde::Deserialize;
use tracing::info;

use crate::{
    manifest::{Bugs, Human, Person, Repository},
    PackageJson,
};

#[derive(Debug, Deserialize)]
struct Dist {
    shasum: String,
    tarball: String,
    integrity: Option<String>,
    #[serde(rename = "fileCount")]
    file_count: Option<u32>,
    #[serde(rename = "unpackedSize")]
    unpacked_size: Option<u32>,
    #[serde(rename = "npm-signature")]
    npm_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VersionMeta {
    _from: Option<String>,
    _id: String,
    #[serde(rename = "_nodeVersion")]
    _node_version: String,
    #[serde(rename = "_npmUser")]
    _npm_user: Person,
    #[serde(rename = "_npmVersion")]
    _npm_version: String,
    _shasum: Option<String>,
    #[serde(rename = "_hasShrinkwrap")]
    _has_shrinkwrap: Option<bool>,
    dist: Dist,
    files: Vec<String>,

    #[serde(flatten)]
    package_json: PackageJson,
}

#[derive(Debug, Deserialize)]
struct PackageMeta {
    _id: String,
    _rev: String,
    #[serde(rename = "dist-tags")]
    dist_tags: IndexMap<String, String>,
    name: String,
    time: IndexMap<String, String>,
    users: IndexMap<String, bool>,
    versions: IndexMap<String, VersionMeta>,

    author: Human,
    bugs: Option<Bugs>,
    contributors: Option<Vec<Human>>,
    description: Option<String>,
    homepage: Option<String>,
    keywords: Option<Vec<String>>,
    license: Option<String>,
    maintainers: Option<Vec<Human>>,
    readme: Option<String>,
    #[serde(rename = "readmeFilename")]
    readme_filename: Option<String>,
    repository: Option<Repository>,
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
    pub fn inspect_package(&self, name: &str) -> Result<()> {
        let res = self.reqwest.get(format!("{BASE_URL}/{name}")).send()?;
        let code = res.status();
        let body = res.text()?;
        let meta = serde_json::from_str::<PackageMeta>(&body);
        if let Err(err) = &meta {
            tracing::error!(?err, "error");
            let col = err.column();
            let after = &body[col..][..10];
            let before = &body[col - 100..col];
            tracing::error!(%before, %after, "err");
        }

        info!(?code, ?meta, "Received response");
        Ok(())
    }
}
