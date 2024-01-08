use actix_web::{get, middleware::Logger, App, HttpRequest, HttpResponse, HttpServer, Responder};
use env_logger::Env;

#[get("/")]
async fn handler(req: HttpRequest) -> impl Responder {
    let cookie = req.headers().get("cookie");
    println!("req: {:?}, cookie: {:?}", req, cookie);

    match cookie {
        Some(_) => HttpResponse::Ok().body("<html><body>hello, again</body></html>\n"),
        None => HttpResponse::Ok().body("<html><body>hello, first time!</body></html>\n"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!("start http listening :18888");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i %{Referer}i"))
            .service(handler)
    })
    .bind(("127.0.0.1", 18888))?
    .run()
    .await
}
