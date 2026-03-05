use anyhow::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

use crate::config::FaqFile;

/// A discovered .faq file and its parsed content
#[derive(Debug)]
pub struct DiscoveredFaq {
    pub path: PathBuf,
    pub faq: FaqFile,
}

/// Discover all .faq files under the given root directory, respecting .gitignore
pub fn discover(root: &Path) -> Result<Vec<DiscoveredFaq>> {
    let mut results = Vec::new();

    for entry in WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .build()
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "faq") {
            match FaqFile::load(path) {
                Ok(faq) => {
                    tracing::debug!("Discovered FAQ: {}", path.display());
                    results.push(DiscoveredFaq {
                        path: path.to_path_buf(),
                        faq,
                    });
                }
                Err(e) => {
                    tracing::warn!("Failed to parse {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(results)
}
