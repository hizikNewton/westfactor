use actix_web::{dev::Server, web::Data, App, HttpServer};
use std::net::TcpListener;

use crate::routers::routes::general_routes;

pub async fn run<T: 'static + Send + Sync>(
    listener: TcpListener,
    shared_data: Data<T>,
) -> Result<Server, std::io::Error> {
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            /* .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                EzyTutorError::InvalidInput("Please provide valid Json input".to_string()).into()
            })) */
            .configure(general_routes)
    };
    let server = HttpServer::new(app).listen(listener)?.run();
    Ok(server)
}
