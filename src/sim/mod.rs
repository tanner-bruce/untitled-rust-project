use bevy::app::App;
use bevy::ecs::event::Events;
use bevy::prelude::{Plugin, World};
use crate::sim::generator::WorldOptions;
use crate::sim::world::data::EntityData;
use crate::sim::orders::*;
use crate::sim::world::tilemap::{TileChunk, TileLayer};

pub mod world;
pub mod generator;
pub mod orders;

#[derive(Default)]
pub struct Sim {
    ground: TileLayer<u16>
}

// Jobs captures all created jobs and stores them to be used when ticking
// pub struct Jobs {
//     pub list: Vec<Job>
// }

impl Sim {
    pub fn new(mut w: &mut World, opts: WorldOptions) -> Self {
        w.insert_resource(opts.clone());

        Orders::add_to_world(w);

        Self{
            ..Default::default()
        }
    }

    /// Generate the world
    pub fn generate(&mut self) {
        // create mountains
        // create rivers
        // create forests
        // create mineral formations
        // create artificial places
        // create tiles
        let mut c = self.ground.chunk([0, 0]);
        c.data.iter_mut().for_each(|f| {
        });
    }
}

pub struct System;

impl Plugin for System {
    fn build(&self, app: &mut App) {
        app.add_plugin(orders::System)
            .add_plugin(world::entity::System); // does nothing
    }
}