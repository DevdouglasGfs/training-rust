pub fn is_validate_username(name: String) -> bool {
    let is_filled = !name.trim().is_empty();
    let has_number = name.chars().any(|char| !!char.is_numeric());
    return !has_number && is_filled;
}
