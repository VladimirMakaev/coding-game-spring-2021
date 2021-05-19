use std::path::Path;

use bundle_rs::Bundle;

fn main() -> std::io::Result<()> {
    Bundle::new(
        Path::new("./src/main.rs"),
        Path::new("./dist/singlefile.rs"),
    )
    .stript_tests(true)
    .build_output()?;
    Ok(())
}
