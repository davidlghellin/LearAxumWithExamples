pub async fn hello_word() -> String {
    "Hola mundo desde mi fichero".to_owned()
}

#[tokio::test]
async fn test_something_async() {
    assert_eq!(hello_word().await, "Hola mundo desde mi fichero".to_string())
}
