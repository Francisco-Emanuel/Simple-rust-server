use actix_web::{get, HttpResponse, Responder};
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
    context.insert("hello_from_rust", "this comes from rust!");
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/ping")]
async fn pong() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}