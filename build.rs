fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/dist_fs.proto")?;
    tonic_prost_build::compile_protos("proto/greeter.proto")?;
    Ok(())
}
