//= USES ===========================================================================================

use std::env;

use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;

//= MAIN ===========================================================================================

fn main() -> Result<()> {
    // This tells cargo to rerun this script if something in /res/ changes.
    println!("cargo:rerun-if-changed=res/*");

    let paths_to_copy = vec!["res/"];
    let out_dir = env::var("OUT_DIR")?;
    let copy_options = CopyOptions {
        overwrite: true,
        ..Default::default()
    };
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    Ok(())
}
