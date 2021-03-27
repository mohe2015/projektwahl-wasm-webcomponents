use actix_files as fs;
use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

// https://localhost:8443/index.html

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("nopass.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new().service(fs::Files::new("/", "..").prefer_utf8(true))
    })
    .bind_openssl("127.0.0.1:8443", builder)?
    .bind("127.0.0.1:8080")?
    .run()
    .await
}