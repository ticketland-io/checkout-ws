use actix_cors::Cors;
use actix_web::{middleware, web, http, App, HttpResponse, HttpServer};
use env_logger::Env;
use std::{env, panic, process};
use ticketland_ws::{
  utils::store::Store,
  ws::entrypoint::ws_index,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let orig_hook = panic::take_hook();
  panic::set_hook(Box::new(move |panic_info| {
    orig_hook(panic_info);
    process::exit(1);
  }));

  if env::var("ENV").unwrap() == "development" {
    dotenv::from_filename(".env").expect("cannot load env from a file");
  }

  // Initialize the logger to use environment variables.
  // In this case, a good default is setting the environment variable
  // `RUST_LOG` to `debug`.
  tracing_subscriber::fmt::init();

  let store = web::Data::new(Store::new().await);
  let port = store.config.port;
  let cors_origin = store.config.cors_origin.clone();

  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  HttpServer::new(move || {
    let cors = Cors::default()
      .allowed_origin(&cors_origin)
      .allowed_methods(vec!["GET", "POST"])
      .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
      .allowed_header(http::header::CONTENT_TYPE)
      .max_age(3600);

    App::new()
      .app_data(store.clone())
      .wrap(cors)
      .wrap(middleware::Logger::default())
      .service(web::resource("/ws/").route(web::get().to(ws_index)))
      .route("/", web::get().to(|| HttpResponse::Ok()))
  })
  .bind(format!("0.0.0.0:{}", port))?
  .run()
  .await
}
