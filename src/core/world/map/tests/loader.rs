use crate::core::world::map::loader::MapItemPrototypesLoader;

#[test]
fn load() {
    let mut loader = MapItemPrototypesLoader::new();
    let load_result = loader.load();
    assert!(load_result.is_ok());
}
