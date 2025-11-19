fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();

        // Get version from Cargo.toml
        let version = env!("CARGO_PKG_VERSION");

        res.set("ProductName", "Sublime Shims")
            .set("CompanyName", "Eric Bouchard")
            .set("LegalCopyright", "Copyright © 2025")
            .set("ProductVersion", version)
            .set("FileVersion", version)
            .set("FileDescription", "Sublime Merge shim");

        res.compile().unwrap();
    }
}
