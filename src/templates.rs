use askama::Template;


#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate {}

#[derive(Template)]
#[template(path = "paste.html")]
pub struct PasteTemplate {
    pub id: String,
    pub content: String
}