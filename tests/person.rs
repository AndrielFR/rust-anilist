use rust_anilist::Client;

#[tokio::test]
async fn get_person() {
    let person = Client::default().get_person(96879).await;
    assert!(person.is_ok())
}
