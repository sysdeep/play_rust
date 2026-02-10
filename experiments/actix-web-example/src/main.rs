use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual() -> impl Responder {
    HttpResponse::Ok().body("manual")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // service
            .service(hello)
            .service(echo)
            // custom service
            .service(web::scope("/api").route("lala", web::get().to(manual)))
            // custom route
            .route("/hey", web::get().to(manual))
    })
    .bind(("127.0.0.1", 8877))?
    .run()
    .await
}
