extern crate easyadmin;
use easyadmin::view::AdminView;
use std::collections::HashMap;

#[get("/", format = "text/html")]
pub fn dashboard() -> Option<AdminView> {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("version", "0.1.0");
    let mut view: AdminView = AdminView::new("dashboard");
    view.set(data);
    Some(view)
}
