use rocket_contrib::templates::{tera::Context, Template};

#[get("/")]
pub fn get() -> Template {
    Template::render("index", Context::default())
}
