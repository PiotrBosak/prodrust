use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_test() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_fomr_data() {
    //Arrange
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to conenct to Postgres.");

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");

    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    //Act
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute");

    //Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!(saved.email, "ursula_le_guin%gmail.com");
    assert_eq!(saved.name, "le guin");
}
#[actix_rt::test]
async fn subscribe_returns_400_when_data_is_missing() {
    //Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing both"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute");

        //Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not faile when payload was {}",
            error_message
        )
    }

    //Act
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-type", "application/x-www-form-urlencoded")
        .send()
        .await
        .expect("Failed to execute");
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let server = zerotoprod::startup::run(listener).expect("Failed server spawn");
    let _ = tokio::spawn(server);
    format! {"http://localhost:{}", port}
}
