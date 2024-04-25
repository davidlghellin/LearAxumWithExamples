pub async fn hello_word() -> String {
    "Hola mundo desde mi fichero".to_owned()
}

#[tokio::test]
async fn test_something_async() {
    let hola = hello_word().await;
    assert_eq!(hola, "Hola mundo desde mi fichero".to_string())
}
