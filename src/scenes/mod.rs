pub mod cafe;
pub mod city;

/// Returns the names of all available scenes.
pub fn names() -> Vec<&'static str> {
    vec!["city", "cafe"]
}
