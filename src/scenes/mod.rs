pub mod city;

use crate::scene::Scene;

/// Returns all available scenes.
#[allow(dead_code)] // TODO: used by scene cycling (v2) and `--list scenes`
pub fn all() -> Vec<Box<dyn Scene>> {
    vec![Box::new(city::CityScene::new())]
}
/// Returns the scene matching the given name, or the first available scene.
#[allow(dead_code)] // TODO: used by scene cycling (v2)
pub fn by_name(name: &str) -> Box<dyn Scene> {
    for scene in all() {
        if scene.name() == name {
            return scene;
        }
    }
    all().into_iter().next().expect("no scenes available")
}

/// Returns the names of all available scenes.
pub fn names() -> Vec<&'static str> {
    vec!["city"]
}
