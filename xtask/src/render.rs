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
}
