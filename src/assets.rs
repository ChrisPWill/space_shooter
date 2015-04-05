use sprite::Sprite;
use opengl_graphics::Texture;

use std::collections::{HashMap};
use std::path::Path;
use std::rc::Rc;

pub struct SpriteManager {
    sprite_map: HashMap<u64, Sprite<Texture>>,
    deleted_ids: Vec<u64>,
    prev_max_id: u64,
}

impl SpriteManager {
    fn new() -> SpriteManager {
        SpriteManager{sprite_map: HashMap::<u64, Sprite<Texture>>::new(), prev_max_id: 1, deleted_ids: Vec::<u64>::new()}
    }

    fn get(&self, id: &u64) -> Option<&Sprite<Texture>> {
        self.sprite_map.get(id)
    }

    fn get_mut(&mut self, id: &u64) -> Option<&mut Sprite<Texture>> {
        self.sprite_map.get_mut(id)
    }

    fn load_from_path(&mut self, path: &Path) -> Result<u64, &str> {
        let deleted_ids = &mut self.deleted_ids;
        let prev_max_id = &mut self.prev_max_id;

        let id = 
            if deleted_ids.is_empty() { 
                if *prev_max_id == u64::max_value() { return Err("no available ids") }
                else { *prev_max_id += 1; prev_max_id.clone() }
            } else {
                deleted_ids.pop().expect("couldn't get deleted id")
            };

        let tex = Rc::new(
            match Texture::from_path(&path) {
                Ok(tex) => tex,
                Err(_)  => { return Err("couldn't load texture from path");},
            }
            );

        let sprite = Sprite::from_texture(tex.clone());

        match self.sprite_map.insert(id.clone(), sprite) {
            None => Ok(id),
            Some(_) => Err("existing sprite at id error"),
        }
    }
}

pub struct AssetManager {
    pub sprites: SpriteManager,
}

impl AssetManager {
    fn new() -> AssetManager {
        AssetManager{sprites: SpriteManager::new()}
    }
}
