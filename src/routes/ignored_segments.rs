#[get("/foo/<_>/bar")]
pub fn foo_bar() -> String {
    format!("Đường dẫn có dạng /foo/*/bar", )
}

#[get("/<_..>")]
pub fn everything() -> String {
    format!("Đường dẫn có dạng /*", )
}