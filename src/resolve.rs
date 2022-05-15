use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use color_eyre::{eyre::eyre, Result};
use node_semver::Range;
use tracing::{debug, info};

use crate::{download::PackageMeta, NpmClient, WrapErr};

#[derive(Clone)]
pub struct ResolveContext {
    meta_cache: Arc<RwLock<HashMap<String, Arc<PackageMeta>>>>,
    client: NpmClient,
}

impl ResolveContext {
    pub fn new() -> Self {
        Self {
            meta_cache: Arc::new(Default::default()),
            client: NpmClient::default(),
        }
    }

    async fn get_meta(&self, name: &str) -> Result<Arc<PackageMeta>> {
        {
            let cache_read = self.meta_cache.read().unwrap();
            if let Some(meta) = cache_read.get(name) {
                return Ok(Arc::clone(meta));
            }
        }

        debug!("Fetching package info..");

        // two futures might race here - who cares
        let meta = self
            .client
            .fetch_package_meta(name)
            .await
            .wrap_err("fetching package metadata")?;

        let meta = Arc::new(meta);

        let mut cache_write = self.meta_cache.write().unwrap();
        cache_write.insert(name.to_owned(), Arc::clone(&meta));

        Ok(meta)
    }

    #[tracing::instrument(skip(self, requested_version), fields(requested_version = %requested_version))]
    pub async fn download_package_and_deps(
        &self,
        name: &str,
        requested_version: &Range,
    ) -> Result<()> {
        let meta = self.get_meta(name).await?;

        info!(versions = ?meta.versions.keys().map(ToString::to_string).collect::<Vec<_>>());

        let chosen = meta
            .versions
            .keys()
            .filter(|version| version.satisfies(requested_version))
            .max();

        let version = chosen
            .ok_or_else(|| eyre!("could not find matching version for '{requested_version}'"))?;

        let package = &meta.versions[version];

        info!(%version, "Found version");
        self.client
            .download_package(name, &package.dist.tarball)
            .await
            .wrap_err("downloading package")?;

        Ok(())
    }
}
