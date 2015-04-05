use sprite::Sprite;
use opengl_graphics::Texture;

use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub struct AssetManager {
    pub sprites: SpriteManager,
}

pub struct SpriteManager {
    sprite_map: HashMap<u64, Sprite<Texture>>,
}

impl<'a> Index<&'a u64> for SpriteManager {
    type Output = Sprite<Texture>;

    fn index(&self, _index: &u64) -> &Sprite<Texture> {
        self.sprite_map.get(_index).expect("key not present")
    }
}

impl<'a> IndexMut<&'a u64> for SpriteManager {
    fn index_mut(&mut self, _index: &u64) -> &mut Sprite<Texture> {
        self.sprite_map.get_mut(_index).expect("key not present")
    }
}
