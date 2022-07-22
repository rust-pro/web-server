#[get("/user/<id>")]
pub fn user_usize(id: usize) -> String {
    format!("ID người dùng là một số nguyên dương: {}", id)
}

#[get("/user/<id>", rank = 2)]
pub fn user_isize(id: isize) -> String {
    format!("ID người dùng là một số nguyên âm: {}", id)
}

#[get("/user/<id>", rank = 3)]
pub fn user_str(id: &str) -> String {
    format!("ID người dùng là một chuỗi String: {}", id)
}
