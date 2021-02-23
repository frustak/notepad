use rocket_contrib::templates::{tera::Context, Template};

#[get("/error")]
pub fn get() -> Template {
    Template::render("error", Context::default())
}
