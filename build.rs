fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/jaeger/storage/v2/trace_storage.proto");
    println!("cargo:rerun-if-changed=proto/jaeger/storage/v2/dependency_storage.proto");

    tonic_prost_build::configure()
        .build_client(false)
        .extern_path(
            ".opentelemetry.proto.trace.v1",
            "::opentelemetry_proto::tonic::trace::v1",
        )
        .compile_protos(
            &[
                "proto/jaeger/storage/v2/trace_storage.proto",
                "proto/jaeger/storage/v2/dependency_storage.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
