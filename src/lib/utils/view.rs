use handlebars::Handlebars;
use std::io::Cursor;
use std::collections::HashMap;
use std::path::Path;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;
#[derive(Debug)]
pub struct AdminView {
    content: String,
    template: String,
}

impl AdminView {
    pub fn new(template_name: &str) -> Self {
        AdminView {
            content: String::new(),
            template: String::from(template_name),
        }
    }
    pub fn set(&mut self, data: HashMap<&str, &str>) {
        let mut reg = Handlebars::new();
        reg.register_templates_directory(".tpl", Path::new("views/admin")).unwrap();
        self.content = reg.render(&self.template, &json!(data)).unwrap();
    }
}

impl<'r> Responder<'r> for AdminView {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!("{}", self.content)))
            .header(ContentType::new("text", "html"))
            .ok()
    }
}

#[derive(Debug)]
pub struct FrontendView {
    content: String,
    template: String,
}

impl FrontendView {
    pub fn new(template_name: &str) -> Self {
        FrontendView {
            content: String::new(),
            template: String::from(template_name),
        }
    }
    pub fn set(&mut self, data: HashMap<&str, &str>) {
        let mut reg = Handlebars::new();
        reg.register_templates_directory(".tpl", Path::new("views/frontend")).unwrap();
        self.content = reg.render(&self.template, &json!(data)).unwrap();
    }
}

impl<'r> Responder<'r> for FrontendView {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!("{}", self.content)))
            .header(ContentType::new("text", "html"))
            .ok()
    }
}