use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build protocol buffer definitions if they exist
    let proto_files = [
        "proto/integration.proto",
        "proto/ectus_bridge.proto",
        "proto/unified_orchestrator.proto",
    ];

    let mut found_protos = Vec::new();
    for proto_file in &proto_files {
        let proto_path = PathBuf::from(proto_file);
        if proto_path.exists() {
            found_protos.push(proto_path);
        }
    }

    if !found_protos.is_empty() {
        tonic_build::configure()
            .build_server(true)
            .build_client(true)
            .compile(&found_protos, &["proto"])?;
    }

    // Set environment variables for integration configuration
    println!("cargo:rustc-env=INTEGRATION_MODULE_VERSION=1.0.0");
    println!("cargo:rustc-env=ECTUS_BRIDGE_PROTOCOL_VERSION=1.0.0");
    println!("cargo:rustc-env=MAX_AUTONOMY_LEVEL=255");

    // Rerun if build files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=proto/");

    Ok(())
}