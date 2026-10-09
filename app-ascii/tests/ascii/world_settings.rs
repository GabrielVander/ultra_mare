use app_ascii::{Glyph, generate_world, glyph_for, present_map};
use lib_terrain::TerrainType;
use lib_world_generation::WorldMap;

#[test]
fn should_generate_the_same_world_for_the_same_seed() {
    let first: WorldMap = generate_world(7);
    let second: WorldMap = generate_world(7);

    assert_eq!(present_map(&first), present_map(&second));
}

#[test]
fn should_generate_both_sea_and_land() {
    let sea: Glyph = glyph_for(TerrainType::Sea);

    [1, 2, 3].into_iter().for_each(|seed| {
        let glyphs: Vec<Glyph> = present_map(&generate_world(seed))
            .rows
            .into_iter()
            .flatten()
            .collect();

        assert!(glyphs.contains(&sea), "seed {seed}");
        assert!(glyphs.iter().any(|glyph| *glyph != sea), "seed {seed}");
    });
}
