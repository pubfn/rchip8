fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cpp/display_qt.cpp");
    println!("cargo:rustc-check-cfg=cfg(has_qt_cpp)");

    let qt_probe = pkg_config::Config::new()
        .atleast_version("5")
        .probe("Qt5Widgets");

    if let Ok(library) = qt_probe {
        let mut build = cc::Build::new();
        build.cpp(true);
        build.file("cpp/display_qt.cpp");
        build.flag_if_supported("-std=c++17");

        for include_path in &library.include_paths {
            build.include(include_path);
        }

        build.compile("chip8_qt_bridge");
        println!("cargo:rustc-cfg=has_qt_cpp");
    }
}
