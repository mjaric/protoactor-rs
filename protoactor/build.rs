fn main() -> std::io::Result<()> {
    let mut prost_build = prost_build::Config::new();
    prost_build.compile_protos(&["src/protos.proto"], &["src"])?;

    Ok(())
}
