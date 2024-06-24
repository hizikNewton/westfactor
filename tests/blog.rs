use actix_rt::net::TcpListener;
use westfactor::startup::run;


struct TestApp {
    address:String,
    db_pool:String
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = actix_rt::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}