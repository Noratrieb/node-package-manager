use color_eyre::{eyre::WrapErr, Result};
use tokio::fs;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

use crate::{
    download::NpmClient, helper::create_dir_if_not_exists, manifest::PackageJson,
    resolve::ResolveContext,
};

mod download;
mod helper;
mod manifest;
mod resolve;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    setup_tracing()?;

    let manifest = "package.json";
    let manifest = fs::read_to_string(manifest)
        .await
        .wrap_err("Opening package.json file")?;

    let manifest: PackageJson = serde_json::from_str(&manifest)?;

    let resolve_context = ResolveContext::new();

    create_dir_if_not_exists("node_modules").await?;

    for (name, requested_version) in &manifest.dependencies.unwrap() {
        resolve_context
            .download_package_and_deps(name, requested_version)
            .await
            .wrap_err(format!("package {name}"))?;
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
