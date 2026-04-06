use anyhow::*;
use std::path::Path;
use vergen_gix::{Emitter, GixBuilder};

fn main() -> Result<()> {
    if Path::new(".git").exists() {
        Emitter::default()
            .add_instructions(&GixBuilder::all_git()?)?
            .emit()
    } else {
        Ok(())
    }
}
