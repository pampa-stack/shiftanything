use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

fn main() {
    check_catalog();
    // Signed launcher updates exist where an installed package can be replaced safely.
    println!("cargo::rustc-check-cfg=cfg(signed_updates)");
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if os == "macos" || os == "linux" || (os == "windows" && std::env::var_os("CARGO_FEATURE_WINDOWS_LIFECYCLE").is_some()) {
        println!("cargo::rustc-cfg=signed_updates");
    }
    // Windows does not allow * in file names: tauri.windows.json is the one source of the Windows product name,
    // which the NSIS setup uses for the install folder and the uninstall registry key.
    println!("cargo:rerun-if-changed=tauri.windows.json");
    let windows = fs::read_to_string("tauri.windows.json").expect("read Windows Tauri config");
    let windows = serde_json::from_str::<serde_json::Value>(&windows).expect("parse Windows Tauri config");
    let product = windows["productName"].as_str().expect("Windows productName");
    assert_eq!(windows["mainBinaryName"].as_str(), Some(product), "Windows mainBinaryName must equal productName");
    println!("cargo::rustc-env=SHIFT_WINDOWS_PRODUCT={product}");
    tauri_build::build()
}

fn check_catalog() {
    let catalog_path = "../../crates/core/curated.json";
    println!("cargo:rerun-if-changed={catalog_path}");
    let catalog = fs::read_to_string(catalog_path).expect("read Rust catalog");
    let apps = serde_json::from_str::<Vec<serde_json::Value>>(&catalog).expect("parse Rust catalog");
    let ids: BTreeSet<_> = apps.iter().map(|app| app["id"].as_str().expect("app ID")).collect();
    assert_eq!(ids.len(), apps.len(), "duplicate app ID in Rust catalog");

    for name in ["index", "it", "en"] {
        let path = format!("frontend/{name}.html");
        println!("cargo:rerun-if-changed={path}");
        let page = fs::read_to_string(&path).expect("read catalog page");
        for (prefix, occurrences) in [(" data-app=\"", 2), ("<template id=\"app-detail-", 1)] {
            let mut actual = BTreeMap::new();
            for entry in page.split(prefix).skip(1) {
                let (id, _) = entry.split_once('"').expect("catalog element ID");
                *actual.entry(id).or_insert(0) += 1;
            }
            let expected = ids.iter().map(|&id| (id, occurrences)).collect();
            assert!(
                actual == expected,
                "{path} disagrees with the Rust catalog at {prefix}. Refresh with tools/export_tauri_proof.py <design-dir> --catalog-only."
            );
        }
    }
}
