pub fn sanitize_for_path(path: String) -> String {
    path.trim()
        .replace(":", "")
        .replace("/", "")
        .replace("\\", "")
        .replace("*", "")
        .replace("?", "")
        .replace("\"", "")
        .replace("<", "")
        .replace(">", "")
        .replace("|", "")
        .to_owned()
}

pub fn sanitize_from_path(path: String) -> String {
    path.trim()
        .replace("(", "")
        .replace(")", "")
        .replace(" ", "-")
        .replace("!", "")
        .replace("/", "")
        .replace(".", "")
        .replace("'", "")
        .to_lowercase()
        .to_owned()
}
