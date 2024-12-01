pub fn hello() -> String {
   return String::from("Hello, world!"); 
}

#[test]
fn test_hello() {
    let result = hello();
    assert_eq!(result, "Hello, world!");
}
