use std::io::Error;

use protoc_bin_vendored::protoc_bin_path;
use walkdir::WalkDir;

fn main() -> Result<(), Error> {
    println!("Compiling protoc...");
    std::env::set_var("PROTOC", protoc_bin_path().unwrap());

    let mut inputs = Vec::new();
    for entry in WalkDir::new("proto").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(filename) = entry.path().file_name().and_then(|name| name.to_str()) {
                if filename.ends_with(".proto") {
                    inputs.push(entry.path().to_str().unwrap().to_string());
                }
            }
        }
    }
    println!("Compiling protobuf");
    let mut pb_cfg = prost_build::Config::new();
    pb_cfg.with_custom_derives(
        vec![
            "serde::Deserialize".to_string(),
            "serde::Serialize".to_string(),
        ],
        vec![
            "serde::Deserialize".to_string(),
            "serde::Serialize".to_string(),
        ],
        vec![
            "serde::Deserialize".to_string(),
            "serde::Serialize".to_string(),
        ],
    );
    pb_cfg.compile_protos(inputs.as_slice(), &["proto"])?;
    Ok(())
}
