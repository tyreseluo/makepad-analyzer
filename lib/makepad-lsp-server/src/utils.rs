use std::path::Path;
use anyhow::Result;
use lsp_types::{CompletionItem, CompletionItemKind, CompletionParams, Url};
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

/// Handle the completion request
pub fn handle_completion(params: CompletionParams) -> Vec<CompletionItem> {
    eprintln!("Get completion request: {:#?}", params);

    let trigger_text = params.context.and_then(|ctx| ctx.trigger_character).unwrap_or_default();

    let current_line = params.text_document_position.text_document.uri.to_string();
    eprintln!("Current line: {}", current_line);

    let keywords = vec![
        "use",
        "fn",
        "link",
        "pub",
        "crate::",
        "dep",
    ];

    let mut items = Vec::<CompletionItem>::new();

    for keyword in keywords {
        if keyword.starts_with(&trigger_text) {
            items.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some(format!("Keyword starting with '{}'", trigger_text)),
                insert_text: Some(format!("{} ", keyword)),
                ..Default::default()
            });
        }
    }

    items
}
