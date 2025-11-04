fn main() {
    // Enable Windows subsystem for release builds
    #[cfg(windows)]
    {
        if std::env::var("PROFILE").unwrap() == "release" {
            println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");
            println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
        }
    }
}
