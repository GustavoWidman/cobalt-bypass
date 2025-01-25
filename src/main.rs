use actix_web::{dev::Service, App, HttpServer};
use log::info;
use utils::Logger;

#[macro_use]
extern crate dotenvy_macro;

mod api;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Logger::init(None);

    let host = dotenv!("HOST");
    let port = dotenv!("PORT");

    info!("Running on http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                info!(
                    "{} {} {:?} {} {}",
                    req.method(),
                    req.path(),
                    req.version(),
                    req.peer_addr()
                        .map_or("unknown".to_string(), |addr| addr.to_string()),
                    req.headers()
                        .get("User-Agent")
                        .map_or("unknown", |s| s.to_str().unwrap_or("unknown")),
                );

                srv.call(req)
            })
            .service(api::get)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
