extern crate pkg_config;

fn main() {
    let library = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("libzfs").unwrap();

    for path in library.link_paths {
        println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
    }
//    panic!("NOOEZ");
}
