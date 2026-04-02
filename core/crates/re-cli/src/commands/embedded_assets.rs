//! Shared helpers for materializing embedded plugin assets.

use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use crate::{CliError, i18n};

pub(super) struct MaterializedAsset<'a> {
    pub path: &'a str,
    pub contents: &'a str,
}

pub(super) fn materialize_assets(
    assets: &[MaterializedAsset<'_>],
    output_dir: &Path,
    locale: &str,
) -> Result<String, CliError> {
    let mut lines = vec![i18n::materialized_assets_heading(locale, assets.len())];

    for asset in assets {
        let output_path = resolve_safe_output_path(output_dir, asset.path, locale)?;
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                CliError::new(i18n::failed_to_write_output(
                    locale,
                    &output_path.display().to_string(),
                    &error.to_string(),
                ))
            })?;
        }
        fs::write(&output_path, asset.contents).map_err(|error| {
            CliError::new(i18n::failed_to_write_output(
                locale,
                &output_path.display().to_string(),
                &error.to_string(),
            ))
        })?;
        lines.push(output_path.display().to_string());
    }

    Ok(lines.join("\n"))
}

fn resolve_safe_output_path(
    output_dir: &Path,
    asset_path: &str,
    locale: &str,
) -> Result<PathBuf, CliError> {
    let relative_path = Path::new(asset_path);
    let mut output_path = PathBuf::from(output_dir);

    for component in relative_path.components() {
        match component {
            Component::Normal(segment) => output_path.push(segment),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(CliError::new(i18n::invalid_embedded_asset_path(
                    locale, asset_path,
                )));
            }
        }
    }

    Ok(output_path)
}
