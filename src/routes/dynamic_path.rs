//Đường dẫn động
#[get("/info/<name>/<age>/<cool>")]
pub fn dynamic_path(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {} !", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}