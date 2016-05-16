extern crate pkg_config;

fn main() {
    // By setting cargo_metadata to false, we and just outputting the link
    // paths, we prevent Cargo from linking to libzfs and libzfs_core as well.

    let library = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("libzfs").unwrap();

    for path in library.link_paths {
        println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
    }
}
