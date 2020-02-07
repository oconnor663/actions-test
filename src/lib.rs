extern "C" {
    pub fn triple(x: u64) -> u64;
}

#[test]
fn test_c_triple() {
    unsafe {
        assert_eq!(triple(5), 15);
    }
}
