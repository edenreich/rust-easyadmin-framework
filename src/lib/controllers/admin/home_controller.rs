use std::collections::HashMap;

use crate::utils;
use utils::view::AdminView;

#[get("/", format = "text/html")]
pub fn index() -> Option<AdminView> {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("name", "eden");
    let mut view: AdminView = AdminView::new("index");
    view.set(data);
    Some(view)
}