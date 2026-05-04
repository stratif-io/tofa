use clap::{Arg, Command};

pub fn render_help_block(cmd: &Command) -> String {
    let name = cmd.get_name();
    let args: Vec<&Arg> = cmd
        .get_arguments()
        .filter(|a| a.get_long().is_some() && !a.is_global_set())
        .collect();
    let synopsis = if args.is_empty() {
        format!("tofa {name}")
    } else {
        format!("tofa {name} [FLAGS]")
    };
    let mut out = format!("**Synopsis**\n\n```\n{synopsis}\n```\n");
    if !args.is_empty() {
        out.push_str("\n**Flags**\n\n| Flag | Description |\n|---|---|\n");
        for a in args {
            let long = a.get_long().unwrap();
            let cell = match a.get_value_names() {
                Some(names) if !names.is_empty() => format!("`--{long} <{}>`", names[0]),
                _ => format!("`--{long}`"),
            };
            let help = a.get_help().map(|h| h.to_string()).unwrap_or_default();
            out.push_str(&format!("| {cell} | {help} |\n"));
        }
    }
    out
}

pub fn replace_block(content: &str, new_inner: &str) -> String {
    const BEGIN: &str = "<!-- BEGIN auto:help -->";
    const END: &str = "<!-- END auto:help -->";
    if let Some(b) = content.find(BEGIN) {
        let begin_end = b + BEGIN.len();
        if let Some(rel) = content[begin_end..].find(END) {
            let e = begin_end + rel;
            let before = &content[..begin_end];
            let after = &content[e..];
            return format!("{before}\n{new_inner}\n{after}");
        }
    }
    // Markers missing: append after first H1, or at end.
    let mut out = String::new();
    let mut inserted = false;
    for line in content.lines() {
        out.push_str(line);
        out.push('\n');
        if !inserted && line.starts_with("# ") {
            out.push_str(&format!("\n{BEGIN}\n{new_inner}\n{END}\n"));
            inserted = true;
        }
    }
    if !inserted {
        out.push_str(&format!("\n{BEGIN}\n{new_inner}\n{END}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synopsis_for_command_with_no_args() {
        let cmd = Command::new("rekey");
        let out = render_help_block(&cmd);
        assert!(out.contains("**Synopsis**"));
        assert!(out.contains("tofa rekey"));
        assert!(!out.contains("[FLAGS]"));
    }

    #[test]
    fn flags_table_lists_each_long_arg() {
        let cmd = Command::new("add")
            .arg(
                Arg::new("name")
                    .long("name")
                    .value_name("NAME")
                    .help("Account name"),
            )
            .arg(
                Arg::new("secret")
                    .long("secret")
                    .value_name("BASE32")
                    .help("TOTP secret"),
            );
        let out = render_help_block(&cmd);
        assert!(out.contains("**Flags**"));
        assert!(out.contains("| `--name <NAME>` | Account name |"));
        assert!(out.contains("| `--secret <BASE32>` | TOTP secret |"));
        assert!(out.contains("tofa add [FLAGS]"));
    }

    #[test]
    fn global_args_are_excluded_from_flags_table() {
        let cmd = Command::new("list").arg(
            Arg::new("vault")
                .long("vault")
                .global(true)
                .help("Vault path"),
        );
        let out = render_help_block(&cmd);
        assert!(!out.contains("--vault"));
        assert!(!out.contains("**Flags**"));
    }

    #[test]
    fn replace_block_swaps_inner_content() {
        let before = "# tofa add\n\nIntent.\n\n<!-- BEGIN auto:help -->\nold\n<!-- END auto:help -->\n\n## Examples\n\nstuff\n";
        let new = "NEW BLOCK";
        let after = replace_block(before, new);
        assert!(after.contains("Intent."));
        assert!(after.contains("## Examples"));
        assert!(after.contains("stuff"));
        assert!(after.contains("<!-- BEGIN auto:help -->\nNEW BLOCK\n<!-- END auto:help -->"));
        assert!(!after.contains("\nold\n"));
    }

    #[test]
    fn replace_block_inserts_after_h1_when_markers_absent() {
        let before = "# tofa add\n\nSome prose.\n";
        let after = replace_block(before, "INNER");
        assert!(after.contains("# tofa add"));
        assert!(after.contains("<!-- BEGIN auto:help -->"));
        assert!(after.contains("INNER"));
        assert!(after.contains("<!-- END auto:help -->"));
        assert!(after.contains("Some prose."));
    }

    #[test]
    fn replace_block_anchors_end_search_after_begin() {
        // A literal mention of the END marker in prose appears before the real block.
        let before = "# tofa add\n\n<!-- END auto:help --> (mentioned in prose)\n\n<!-- BEGIN auto:help -->\nold\n<!-- END auto:help -->\n\nTrailing.\n";
        let after = replace_block(before, "NEW");
        // The block is correctly replaced
        assert!(after.contains("<!-- BEGIN auto:help -->\nNEW\n<!-- END auto:help -->"));
        // The prose mentioning END is preserved (not slurped into the slice)
        assert!(after.contains("(mentioned in prose)"));
        // Trailing content survives
        assert!(after.contains("Trailing."));
        // Old inner content is gone
        assert!(!after.contains("\nold\n"));
    }
}
