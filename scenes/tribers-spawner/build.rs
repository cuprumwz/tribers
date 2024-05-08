use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["./utils/whisper.proto"], &["utils/"])?;
    Ok(())
}

// fn main() -> Result<()> {
//     prost_build::compile_protos(&["src/items.proto"], &["src/"])?;
//     Ok(())
// }
