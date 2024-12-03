use std::path::Path;
use anyhow::Result;
use lsp_types::Url;
use tokio::fs;

/// Now we just scan the workspace for files containing the `live_design!{}` macro
pub async fn scan_workspace(workspace_uri: Url) -> Result<()> {
    find_live_design_files(workspace_uri).await?;
    Ok(())
}

pub async fn find_live_design_files(workspace_uri: Url) -> Result<()> {
    let path = workspace_uri.to_file_path().map_err(|_| anyhow::anyhow!("Invalid workspace path"))?;

    let entries = walkdir::WalkDir::new(&path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| {
                entry.path().is_file() &&
                entry.path().extension().map_or(false, |ext| ext == "rs")
            });

    let mut found_live_design_macro_flag = false;
    for entry in entries {
        if found_live_design_macro_flag {
            eprintln!("Found live_design macro in the workspace, stopping the search");
            break;
        }
        let file_path = entry.path();
        if has_live_design_macro(file_path).await? { found_live_design_macro_flag = true };
    }
    Ok(())
}

/// Check if the file contains the `live_design!{}` macro
async fn has_live_design_macro(file_path: &Path) -> Result<bool> {
    let content = fs::read_to_string(file_path).await?;
    Ok(content.contains("live_design!{"))
}
