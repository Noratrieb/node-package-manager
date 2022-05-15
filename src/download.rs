#![allow(dead_code)]

use std::path::Path;

use bytes::Buf;
use color_eyre::Result;
use indexmap::IndexMap;
use node_semver::Version;
use reqwest::Client;
use serde::Deserialize;
use tar::Archive;
use tracing::{debug, info};

use crate::{
    manifest::{Bugs, Human, Person, Repository},
    PackageJson, WrapErr,
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
    pub versions: IndexMap<Version, VersionMeta>,
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

#[derive(Default)]
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
    pub async fn fetch_package_meta(&self, name: &str) -> Result<PackageMeta> {
        let res = self
            .reqwest
            .get(format!("{BASE_URL}/{name}"))
            .send()
            .await?;
        let code = res.status();
        let body = res.text().await?;
        let meta = serde_json::from_str::<PackageMeta>(&body)?;

        debug!(?code, ?meta, "Received response");
        Ok(meta)
    }

    #[tracing::instrument(skip(self))]
    pub async fn download_package(&self, name: &str, url: &str) -> Result<()> {
        let response = self
            .reqwest
            .get(url)
            .send()
            .await
            .wrap_err("getting response")?;
        let tarball = response.bytes().await.wrap_err("fetching body")?;

        let tar = flate2::read::GzDecoder::new(tarball.reader());
        let mut archive = tar::Archive::new(tar);
        archive
            .unpack(Path::new("node_modules").join(name))
            .wrap_err("unpack tarball")?;

        info!("successfully downloaded package");

        Ok(())
    }
}
