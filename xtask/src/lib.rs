pub mod render;

use anyhow::{Context, Result};
use clap::CommandFactory;
use std::fs;
use std::path::Path;

pub fn gen_docs_at(base: &Path, check: bool) -> Result<Vec<String>> {
    let ref_dir = base.join("docs/book/src/reference");
    fs::create_dir_all(&ref_dir).with_context(|| format!("creating {}", ref_dir.display()))?;
    let root = tofa::cli::Cli::command();
    let mut diffs: Vec<String> = Vec::new();
    for sub in root.get_subcommands() {
        let name = sub.get_name();
        let block = render::render_help_block(sub);
        let path = ref_dir.join(format!("{name}.md"));
        let new_content = if path.exists() {
            let current = fs::read_to_string(&path)?;
            render::replace_block(&current, &block)
        } else {
            render::scaffold_page(name, &block)
        };
        let on_disk = fs::read_to_string(&path).unwrap_or_default();
        if on_disk != new_content {
            if check {
                diffs.push(path.display().to_string());
            } else {
                fs::write(&path, &new_content)?;
            }
        }
    }
    Ok(diffs)
}
