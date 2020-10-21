use std::collections::HashMap;
use crate::utils::view::FrontendView;

#[get("/", format = "text/html")]
pub fn index() -> Option<FrontendView> {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("name", "eden");
    let mut view: FrontendView = FrontendView::new("index");
    view.set(data);
    Some(view)
}