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
    // Find proto_root and out_dir flexibly depending on working directory
    let candidates = [
        (Path::new("../../../contracts/protos"), Path::new("./src")),
        (Path::new("../../../../contracts/protos"), Path::new("../../src")),
    ];

    let mut selected = None;
    for (proto_path, src_path) in &candidates {
        if proto_path.exists() && src_path.exists() {
            selected = Some((*proto_path, *src_path));
            break;
        }
    }

    let (proto_root, out_dir) = match selected {
        Some(paths) => paths,
        None => {
            eprintln!("Error: Protos directory or src directory not found.");
            std::process::exit(1);
        }
    };

    let services = [
        "billing",
        "identity",
        "indexer",
        "payout",
        "realtime",
        "shared",
        "store",
        "support",
        "user",
        "verification",
        "wallet",
        "webhook",
    ];

    let mut proto_files = Vec::new();
    for service in &services {
        let service_dir = proto_root.join(service);
        collect_proto_files(&service_dir, &mut proto_files)?;
    }

    proto_files.sort();

    println!("Found {} proto files across {} services.", proto_files.len(), services.len());

    let mut config = tonic_prost_build::Config::new();
    config
        .enable_type_names()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp");

    tonic_prost_build::configure()
        .include_file("mod.rs")
        .out_dir(out_dir)
        .compile_with_config(config, &proto_files, &[proto_root.to_path_buf()])?;

    let mod_rs_path = out_dir.join("mod.rs");
    let content = fs::read_to_string(&mod_rs_path)?;

    let mut new_lines = Vec::new();
    for line in content.lines() {
        let mut add_gate = None;
        for service in &services {
            if line.starts_with(&format!("pub mod {} {{", service)) {
                add_gate = Some(format!("#[cfg(feature = \"{}\")]", service));
                break;
            }
        }
        if let Some(gate) = add_gate {
            new_lines.push(gate);
        }
        new_lines.push(line.to_string());
    }

    fs::write(&mod_rs_path, new_lines.join("\n") + "\n")?;
    println!("Successfully regenerated proto code into {:?} and updated mod.rs with feature gates!", out_dir);

    Ok(())
}
