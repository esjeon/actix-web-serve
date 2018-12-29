use actix;
use actix_web::{middleware, server, App, HttpResponse, HttpRequest, Responder};
use actix_web::fs::{StaticFiles, StaticFileConfig};
use actix_web::dev::{AsyncResult};
use env_logger;

#[derive(Default)]
struct LocalStaticFileConfig;
impl StaticFileConfig for LocalStaticFileConfig {
    fn show_index() -> bool {
        true
    }

    fn index_file() -> Option<&'static str> {
        Some("index.html")
    }

    fn default_handler<S: 'static>(req: &HttpRequest<S>) -> AsyncResult<HttpResponse> {
        HttpResponse::NotFound()
        .body("404 Not Found")
        .into()
    }
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let sys = actix::System::new("actix-web-serve");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .handler(
                "/",
                StaticFiles::with_config("./", LocalStaticFileConfig{})
                    .unwrap()
            )
    })
    .bind("127.0.0.1:8080")
    .expect("Can not start server on given IP/Port")
    .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
