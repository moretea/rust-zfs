extern crate zfs;
mod support;

fn main() {
    test_zpool_create_destroy();
}

fn test_zpool_create_destroy() {
    let handle = zfs::Handle::new().unwrap();
    assert_eq!(zfs::zpool::iter().next(), None);
}
