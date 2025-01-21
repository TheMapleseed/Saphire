use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=src/");

    // Check system requirements
    check_system_requirements()?;

    // MacOS specific setup without Homebrew dependency
    if cfg!(target_os = "macos") {
        setup_macos_native()?;
    }

    Ok(())
}

fn check_system_requirements() -> Result<(), Box<dyn std::error::Error>> {
    // Check Rust version
    let rust_version = Command::new("rustc")
        .arg("--version")
        .output()?;
    
    if !String::from_utf8_lossy(&rust_version.stdout).contains("1.70") {
        println!("cargo:warning=Rust 1.70 or higher is required");
    }

    // Check OpenSSL
    if let Err(_) = pkg_config::probe_library("openssl") {
        println!("cargo:warning=OpenSSL development files not found");
    }

    // Check for required system libraries
    let required_libs = [
        "libssl-dev",
        "libx11-dev",
        "libxcb1-dev",
    ];

    for lib in required_libs.iter() {
        if !check_library_installed(lib) {
            println!("cargo:warning=Required library {} not found", lib);
        }
    }

    Ok(())
}

fn setup_macos_native() -> Result<(), Box<dyn std::error::Error>> {
    // Use native MacOS Security framework
    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=Security");
    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=CoreFoundation");
    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=CoreGraphics");

    // Check for Xcode Command Line Tools
    if !Command::new("xcode-select")
        .arg("-p")
        .output()?
        .status
        .success() 
    {
        println!("cargo:warning=Xcode Command Line Tools required for development");
        println!("cargo:warning=Install using: xcode-select --install");
    }

    Ok(())
}

fn setup_security_features() -> Result<(), Box<dyn std::error::Error>> {
    // Enable memory protection
    println!("cargo:rustc-cfg=feature=\"secure-memory\"");
    
    // Enable mTLS
    println!("cargo:rustc-cfg=feature=\"mtls\"");

    // Set security-related compiler flags
    println!("cargo:rustc-link-arg=-fstack-protector-strong");
    println!("cargo:rustc-link-arg=-Wformat");
    println!("cargo:rustc-link-arg=-Wformat-security");
    
    if cfg!(target_os = "macos") {
        setup_macos_security()?;
    }

    Ok(())
}

fn setup_macos_security() -> Result<(), Box<dyn std::error::Error>> {
    // Check for Xcode Command Line Tools
    if !Command::new("xcode-select")
        .arg("-p")
        .output()?
        .status
        .success() 
    {
        println!("cargo:warning=Xcode Command Line Tools not installed");
    }

    // Set macOS-specific security flags
    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=Security");
    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=CoreFoundation");

    Ok(())
}

fn setup_platform_config() -> Result<(), Box<dyn std::error::Error>> {
    let target_os = env::var("CARGO_CFG_TARGET_OS")?;
    
    match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-cfg=feature=\"macos\"");
        },
        _ => {
            println!("cargo:warning=Unsupported operating system");
        }
    }

    Ok(())
}

fn check_library_installed(library: &str) -> bool {
    Command::new("pkg-config")
        .arg(library)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

// Generate version information
fn generate_version_info() -> Result<(), Box<dyn std::error::Error>> {
    let version = env::var("CARGO_PKG_VERSION")?;
    let git_hash = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()?;
    
    let git_hash = String::from_utf8_lossy(&git_hash.stdout);
    let build_date = chrono::Utc::now().to_rfc3339();

    let version_info = format!(
        r#"pub const VERSION: &str = "{}";
           pub const GIT_HASH: &str = "{}";
           pub const BUILD_DATE: &str = "{}";"#,
        version,
        git_hash.trim(),
        build_date
    );

    std::fs::write(
        Path::new(&env::var("OUT_DIR")?).join("version.rs"),
        version_info
    )?;

    Ok(())
} 