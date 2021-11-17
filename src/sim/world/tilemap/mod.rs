use std::collections::HashMap;
use bevy::prelude::{Vec2, IVec2};

/// TileLayer represents a layer of tiles
#[derive(Default)]
pub struct TileLayer<T> where T: Clone + Default {
    data: HashMap<IVec2, TileChunk<T>>,
}

pub struct TileChunk<T> where T: Clone + Default {
    pub data: Vec<Vec<T>>
}

impl<T> Default for TileChunk<T> where T: Clone + Default {
    fn default() -> Self {
        Self{
            data: vec![vec![T::default(); 512]; 512]
        }
    }
}

impl<T> TileLayer<T> where T: Clone + Default {
    pub fn chunk<P: Into<IVec2>>(&mut self, pos: P) -> &mut TileChunk<T> {
        let d = &mut self.data;
        let pos = pos.into();
        let out = d.get_mut(&pos);
        if out.is_none() {
            d.insert(pos, TileChunk::<T>::default());
        }
        d.get_mut(&pos).unwrap()
    }
}

pub struct Tiles {
}

impl Tiles {
    pub fn new_layer<T, S>(self, name: S) -> TileLayer<T>
        where
            T: Clone + Default,
            S: Into<String>
    {
        let tm = name.into();
        TileLayer::<T>::default()
    }
}