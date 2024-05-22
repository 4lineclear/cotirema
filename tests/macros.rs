#[test]
fn macro_works() {
    use cotirema::cotirema;
    let s = cotirema!("a+?", "aaaaaa");
    assert_eq!(s, ["a", "a", "a", "a", "a", "a"]);
}
