use scopeshare::syncshare::SyncShare;

#[test]
fn test_basic_usage() {
    let s = SyncShare::new(42);
    assert_eq!(s.with(|x| *x), 42);
}
