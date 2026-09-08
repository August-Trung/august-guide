use uuid::Uuid;

/// Generate a new UUID v4 as a string
pub fn new_uuid() -> String {
    Uuid::new_v4().to_string()
}
