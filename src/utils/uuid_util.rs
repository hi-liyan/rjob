use uuid::Uuid;

/// Generates a UUID without hyphens.
///
/// # Examples
///
/// ```
/// let uuid = generate_uuid_without_hyphens();
/// println!("UUID without hyphens: {}", uuid);
/// ```
///
/// # Returns
///
/// A `String` representation of the generated UUID without hyphens.
pub fn generate_uuid_without_hyphens() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string().replace("-", "")
}