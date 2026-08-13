use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn collect_proto_files(dir: &Path, protos: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                collect_proto_files(&path, protos)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("proto") {
                protos.push(path);
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../../../contracts/protos");

    let proto_root = Path::new("../../../contracts/protos");

    let services = [
        ("billing", "CARGO_FEATURE_BILLING"),
        ("identity", "CARGO_FEATURE_IDENTITY"),
        ("indexer", "CARGO_FEATURE_INDEXER"),
        ("payout", "CARGO_FEATURE_PAYOUT"),
        ("realtime", "CARGO_FEATURE_REALTIME"),
        ("shared", "CARGO_FEATURE_SHARED"),
        ("store", "CARGO_FEATURE_STORE"),
        ("support", "CARGO_FEATURE_SUPPORT"),
        ("user", "CARGO_FEATURE_USER"),
        ("verification", "CARGO_FEATURE_VERIFICATION"),
        ("wallet", "CARGO_FEATURE_WALLET"),
        ("webhook", "CARGO_FEATURE_WEBHOOK"),
    ];

    let full_active = env::var_os("CARGO_FEATURE_FULL").is_some();

    let mut enabled_services: Vec<&str> = Vec::new();
    for (service_name, feature_env) in &services {
        if full_active || env::var_os(feature_env).is_some() {
            enabled_services.push(service_name);
        }
    }

    let mut proto_files = Vec::new();
    for service in enabled_services {
        let service_dir = proto_root.join(service);
        collect_proto_files(&service_dir, &mut proto_files)?;
    }

    proto_files.sort();

    let out_dir = "./src";
    if !proto_files.is_empty() {
        let mut config = tonic_prost_build::Config::new();
        config
            .enable_type_names()
            .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
            .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp");

        tonic_prost_build::configure()
            .include_file("mod.rs")
            .out_dir(out_dir)
            .compile_with_config(config, &proto_files, &[proto_root.to_path_buf()])?;
    } else {
        fs::write(Path::new(out_dir).join("mod.rs"), "// No proto features enabled\n")?;
    }

    Ok(())
}

