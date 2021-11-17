use bevy::prelude::*;
use bevy::ecs::event::Events;
use bevy_event_set::*;
use rand::{thread_rng, Rng};

/// System creates the events that will be used for the order system. These events have
/// receivers inside the sim itself. The expectation is that something will send these
/// events to add actions to the sim
pub struct System;

impl Plugin for System {
    fn build(&self, app: &mut App) {
        // app.init_resource::<Orders>()
        app.add_event_set::<Orders>()
            .add_system(build_order_handler);
    }
}

event_set!(Orders { BuildOrder, HarvestOrder, MoveOrder, InterfaceOrder });

pub type MoveOrder = InputOrder<Move>;
#[derive(Debug, Default, Copy, Clone)]
pub struct Move {
    pub target: (f32, f32),
    pub entity: u32
}

pub type BuildOrder = InputOrder<Build>;
#[derive(Debug, Default, Copy, Clone)]
pub struct Build {
    pub origin: (f32, f32),
    pub building_id: i32,
    pub rotation: u8,
    pub ghost: bool,
}

pub type HarvestOrder = InputOrder<Harvest>;
#[derive(Debug, Default, Clone)]
pub struct Harvest {
    positions: Vec<(f32, f32)>
}

pub type InterfaceOrder = InputOrder<Interface>;
#[derive(Debug, Default, Clone)]
pub struct Interface {
    pub show_build_menu: bool,
}

// Data is the raw data backing an input
#[derive(Debug, Hash)]
pub struct InputOrder<InputType> {
    pub created_at: u64,
    pub id: u64,
    pub priority: u8,
    pub data: InputType,
}

impl<T> InputOrder<T> {
    pub fn reqs() -> bool {
        true
    }

    pub fn unbox(self) -> T {
        self.data
    }
}

pub trait MakeOrder<T> {
    fn new(data: T) -> Self;
}

impl<T> MakeOrder<T> for InputOrder<T> {
    fn new(data: T) -> Self {
        let mut r = thread_rng();
        InputOrder{
            created_at: r.gen(),
            id: r.gen(),
            priority: 5,
            data
        }
    }
}

fn build_order_handler(mut evs: EventReader<InputOrder<Build>>) {
    for e in evs.iter() {
        let d: Build = e.data;
        let position_is_valid = d.origin > (-1., -2.);
        // if !position_is_valid {
        //     // play dot noise
        //     // return;
        // }
        info!("{:?}", e);
    }
}
