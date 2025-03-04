use std::env;
use std::path::PathBuf;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Print out for debugging purposes
    println!("Target architecture: {}", target_arch);
    println!("Target OS: {}", target_os);

    // Link to the correct framework based on the target
    if target_os == "ios" {
        if target_arch == "aarch64" {
            // For real device (ARM64)
            println!(
                "cargo:rustc-link-search=framework=ONNX_POD/onnxruntime.xcframework/ios-arm64"
            );
            println!("cargo:rustc-link-lib=framework=onnxruntime");
        } else if target_arch == "x86_64" {
            // For simulator (x86_64)
            println!("cargo:rustc-link-search=framework=ONNX_POD/onnxruntime.xcframework/ios-arm64_x86_64-simulator");
            println!("cargo:rustc-link-lib=framework=onnxruntime");
        } else {
            panic!("Unsupported iOS architecture: {}", target_arch);
        }
    } else if target_os == "macos" {
        if target_arch == "aarch64" || target_arch == "x86_64" {
            // For macOS (either ARM64 or x86_64)
            println!("cargo:rustc-link-search=framework=ONNX_POD/onnxruntime.xcframework/macos-arm64_x86_64");
            println!("cargo:rustc-link-lib=framework=onnxruntime");
        } else {
            panic!("Unsupported macOS architecture: {}", target_arch);
        }
    } else {
        panic!("Unsupported OS: {}", target_os);
    }

    /*
    let framework_path = "ONNX_POD/onnxruntime.xcframework/ios-arm64";

    println!("cargo:rustc-link-search=framework={}", framework_path);
    println!("cargo:rustc-link-lib=framework=onnxruntime");

    let headers_path = format!("{}/Headers", framework_path);

     */
}
