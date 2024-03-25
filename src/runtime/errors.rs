pub fn error(line: usize, message: String) -> String{
    report(line, "", message)
}

pub fn report(line: usize, where_in_line: &str, message: String) -> String {
    format!("[line {}] Error{}: {}\n", line, where_in_line, message).to_string()
}
