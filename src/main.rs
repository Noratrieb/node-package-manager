use std::fs;

use color_eyre::{eyre::WrapErr, Result};
use tracing::{debug, metadata::LevelFilter};
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

    for (name, _) in &manifest.dependencies.unwrap() {
        client.inspect_package(name)?;
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
