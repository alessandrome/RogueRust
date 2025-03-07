use crate::core::world::map::loader::MapItemPrototypesLoader;

#[test]
fn load() {
    let mut loader = MapItemPrototypesLoader::new();
    let load_result = loader.load();
    for tile in loader.get_tiles() {
        println!("{:#?}", tile);
    }
    assert!(load_result.is_ok());
}
