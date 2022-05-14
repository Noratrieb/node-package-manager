use std::fs;

use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};
use semver_rs::{Range, Version};
use tracing::{debug, info, metadata::LevelFilter, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

use crate::{download::NpmClient, manifest::PackageJson};

mod download;
mod manifest;

fn main() -> Result<()> {
    color_eyre::install()?;
    setup_tracing()?;

    let manifest = "testing/package.json";
    let manifest = fs::read_to_string(manifest).wrap_err("Opening package.json file")?;

    let manifest: PackageJson = serde_json::from_str(&manifest)?;

    debug!(?manifest, "Read manifest");

    let client = NpmClient::new();

    for (name, requested_version) in &manifest.dependencies.unwrap() {
        look_at_package(name, requested_version, &client).wrap_err(format!("package {name}"))?;
    }

    Ok(())
}

#[tracing::instrument(skip(client))]
fn look_at_package(name: &str, requested_version: &str, client: &NpmClient) -> Result<()> {
    let requested = Range::new(requested_version).parse()?;

    let meta = client.inspect_package(name)?;

    info!(versions = ?meta.versions.keys());

    let mut versions = meta
        .versions
        .keys()
        .map(|v| Ok((v, Version::new(v).parse()?)))
        .collect::<Result<Vec<_>, semver_rs::Error>>()?;

    versions.sort_by(|a, b| b.cmp(a));

    let chosen = versions.iter().find(|(_, version)| requested.test(version));

    match chosen {
        Some((version, _)) => {
            info!(?version, "Found version")
        }
        None => bail!("could not find matching version for '{requested_version}'"),
    }

    Ok(())
}

fn setup_tracing() -> Result<()> {
    let registry = Registry::default().with(
        EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env()?,
    );
    let tree_layer = tracing_tree::HierarchicalLayer::new(2)
        .with_targets(true)
        .with_bracketed_fields(true);

    registry.with(tree_layer).init();
    Ok(())
}
