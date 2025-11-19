fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();

        // Get version from Cargo.toml
        let version = env!("CARGO_PKG_VERSION");

        res.set("CompanyName", "Eric Bouchard")
            .set("FileDescription", "Sublime Text shim")
            .set("FileVersion", version)
            .set("InternalName", "subl")
            .set("LegalCopyright", "Copyright © 2025")
            .set("OriginalFilename", "subl.exe")
            .set("ProductName", "Sublime Shims")
            .set("ProductVersion", version);

        res.compile().unwrap();
    }
}
