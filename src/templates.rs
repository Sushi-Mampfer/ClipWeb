use askama::Template;

#[derive(Template)]
#[template(path = "paste.html")]
pub struct PasteTemplate {
    pub id: String,
    pub content: String
}