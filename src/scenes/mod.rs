pub mod cafe;
pub mod city;
pub mod meadow;

/// Returns the names of all available scenes.
pub fn names() -> Vec<&'static str> {
    vec!["city", "cafe", "meadow"]
}
