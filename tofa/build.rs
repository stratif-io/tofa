fn main() {
    // Man page generation will be done at install time via clap_mangen CLI
    // or can be added later. For now, just ensure build succeeds.
    println!("cargo:rerun-if-changed=src/cli/mod.rs");
}
