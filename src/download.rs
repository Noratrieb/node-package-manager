#![allow(dead_code)]

use color_eyre::Result;
use indexmap::IndexMap;
use reqwest::blocking::Client;
use serde::Deserialize;
use tracing::{error, info};

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    url: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Bugs {
    url: String,
}

#[derive(Debug, Deserialize)]
struct Repository {
    r#type: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct Dist {
    shasum: String,
    tarball: String,
}

// todo: this is actually just a package.json with extra stuff
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
    author: Person,
    bugs: Bugs,
    dependencies: IndexMap<String, String>,
    #[serde(rename = "devDependencies")]
    dev_dependencies: IndexMap<String, String>,
    dist: Dist,
    engines: IndexMap<String, String>,
    files: Vec<String>,
    homepage: String,
    keywords: Vec<String>,
    license: String,
    main: String,
    maintainers: Vec<Person>,
    name: String,
    repository: Repository,
    scripts: IndexMap<String, String>,
    version: String,
}

#[derive(Debug, Deserialize)]
struct PackageMeta {
    _id: String,
    _rev: String,
    author: Person,
    bugs: Bugs,
    #[serde(default = "Vec::new")]
    contributors: Vec<Person>,
    description: String,
    #[serde(rename = "dist-tags")]
    dist_tags: IndexMap<String, String>,
    homepage: String,
    keywords: Vec<String>,
    license: String,
    maintainers: Vec<Person>,
    name: String,
    readme: String,
    #[serde(rename = "readmeFilename")]
    readme_filename: String,
    repository: Repository,
    time: IndexMap<String, String>,
    users: IndexMap<String, bool>,
    versions: IndexMap<String, VersionMeta>,
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
        if let Err(err) = meta {
            error!(?err, "error");
            let col = err.column();
            let after = &body[col..][..50];
            let before = &body[col - 50..col];
            error!(%before, %after, "err");
        }

        info!(?code, "Received response");
        Ok(())
    }
}
