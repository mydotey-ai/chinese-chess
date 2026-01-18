use std::env;
use std::path::PathBuf;

fn main() {
    // 设置 OUT_DIR 环境变量
    let out_dir = env::var_os("OUT_DIR").unwrap_or_else(|| ".".into());
    println!("cargo:rustc-env=OUT_DIR={}", out_dir.to_str().unwrap());

    // 确保 tauri.conf.json 文件的路径正确
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let tauri_conf_path = PathBuf::from(manifest_dir).join("tauri.conf.json");
    if tauri_conf_path.exists() {
        println!(
            "cargo:rerun-if-changed={}",
            tauri_conf_path.to_str().unwrap()
        );
    }
}
