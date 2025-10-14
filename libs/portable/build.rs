fn main() {
    #[cfg(windows)]
    {
        use std::io::Write;
        let mut res = winres::WindowsResource::new();
        res.set_icon("../../res/icon.ico")
            .set_language(winapi::um::winnt::MAKELANGID(
                winapi::um::winnt::LANG_ENGLISH,
                winapi::um::winnt::SUBLANG_ENGLISH_US,
            ))
            .set_manifest_file("../../res/manifest.xml");
        let product_name = option_env!("CUSTOM_APP_NAME").filter(|s| !s.is_empty()).unwrap_or("RustDesk");
        let product_description = option_env!("CUSTOM_APP_DESCRIPTION").filter(|s| !s.is_empty()).unwrap_or("RustDesk Remote Desktop");
        res.set("LegalCopyright", &format!("Copyright © 2025 {}. All rights reserved.", product_name));
        res.set("ProductName", product_name);
        res.set("OriginalFilename", &format!("{}.exe", product_name));
        res.set("FileDescription", product_description);
        match res.compile() {
            Err(e) => {
                write!(std::io::stderr(), "{}", e).unwrap();
                std::process::exit(1);
            }
            Ok(_) => {}
        }
    }
}
