pub fn is_validate_username(name: String) -> bool {
    let is_filled = !name.trim().is_empty();
    let is_letters = name.trim().chars().all(|char| !!char.is_alphabetic());
    return is_filled && is_letters;
}
