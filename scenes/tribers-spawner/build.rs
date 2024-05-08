use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["utils/whisper.proto"], &["src/"])?;
    Ok(())
}
