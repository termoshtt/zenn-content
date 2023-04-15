fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["src/items.proto"], &["src/"])?;
    Ok(())
}
