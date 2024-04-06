use actix_web::{get, post, delete, put, patch, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}

#[get("/")]
async fn index() -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("hello_from_rust", "Ola do rust!");
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

// Tentando fazer o htmx funcionar (Não está afetando o código, mas tambem não faz nada)
#[get("/ping")]
async fn pong() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(pong)
    })
    .bind(("127.0.0.1", 42069))?
    .run()
    .await
}