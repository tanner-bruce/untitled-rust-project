use rand::Rng;
use bevy::prelude::*;
use bevy::app::{App, EventReader};
use bevy::ecs::component::Component;
use bevy::render::draw::{RenderCommand, OutsideFrustum};
use bevy::gltf::{Gltf, GltfPrimitive};
use std::ops::Range;
use bevy::ecs::system::EntityCommands;

pub struct System;

impl Plugin for System {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct CoreAttributes {
    pub constitution: f32,
    pub agility: f32,
    pub dexterity: f32,
    pub strength: f32,
    pub luck: f32,
    pub intelligence: f32,
}

impl CoreAttributes {
    pub fn random() -> Self {
        let mut ca = CoreAttributes::default();
        let mut r = rand::thread_rng();
        let mut pts: i8 = r.gen_range(18..=26);
        while pts > 0 {
            match r.gen_range(0..6) {
                0 => { ca.constitution += 1. },
                1 => { ca.agility += 1. },
                2 => { ca.dexterity += 1. },
                3 => { ca.strength += 1. },
                4 => { ca.luck += 1. },
                5 => { ca.intelligence += 1. },
                _ => (),
            }
            pts -= 1;
        }
        ca
    }
}

/// what are traits, anyway?
///
/// I'm working on getting better at this whole "life coaching thing", which I
/// don't know if I'm just dumb, or what. But one of the things that has come
/// clear to me is that I have no concept of what traits are.
///
/// I know there's a word for traits, but I've always had a mental picture of
/// traits being like little bits of dirt or hair stuck to clothes, kind of
/// invisible things that I'm not aware of. But then if I am aware of them,
/// like when I wash my clothes, they don't seem to go
#[derive(Debug, Default, Clone)]
pub struct Trait {
    pub name: String,
    pub description: String,
    pub modifiers: Vec<Modifier>
}

#[derive(Debug, Default, Clone)]
pub struct Modifier {
    pub skill: Option<String>,
    pub attr: Option<String>,
    pub value: i32,
}

pub struct Race {
    pub name: String,
    pub lifespan: Range<i32>,
}

#[derive(Debug, Default, Clone)]
pub struct Living {
    pub id: String,
    pub attrs: CoreAttributes,
    pub has_job: i64,
    // pub traits: Vec<Trait>,
    pub position: Vec3,
    pub species: String,
}

pub struct OneOf<T,V> {
    t: Option<T>,
    v: Option<V>,
}

impl<T: Clone,V: Clone> Clone for OneOf<T,V> {
    fn clone(&self) -> Self {
        Self{
            t: self.t.clone(),
            v: self.v.clone()
        }
    }
}

type LocArea = OneOf<Vec3, (Vec3, Vec3)>;
impl Default for LocArea {
    fn default() -> Self {
        Self{
            t: None,
            v: None,
        }
    }
}

impl Into<LocArea> for Vec3 {
    fn into(self) -> LocArea{
        LocArea{
            t: Some(self),
            v: None
        }
    }
}
impl Into<LocArea> for (Vec3, Vec3) {
    fn into(self) -> LocArea{
        LocArea{
            t: None,
            v: Some(self)
        }
    }
}

pub struct LivingId(i64);

#[derive(Debug, Bundle, Default)]
pub struct LivingBundle {
    pub living: Living,
}

pub trait SpawnLiving<'w, 's> {
    /// spawn_living will, given a Living component, spawn
    fn spawn_living<'a>(&'a mut self, sr: SpawnRequest) -> EntityCommands<'w, 's, 'a>;
}

#[derive(Clone)]
pub enum EntityTags {
    Neutral,
    Hostile,
    Strong,
    Random,
    RandomStrong,
}

#[derive(Default, Clone)]
pub struct SpawnRequest {
    pub id: String,
    pub race: String,
    pub location: Option<LocArea>,
    pub name: Option<String>,
    pub birthday: Option<i64>,
    pub traits: Option<Vec<Trait>>,
    pub tags: Option<EntityTags>,
    pub living: Option<Living>,
    // pub inventory: Option<Inventory>
}

impl SpawnRequest {
    pub fn with_loc<T: Into<LocArea>>(&mut self, p: T) -> &mut Self {
        self.location = Some(p.into());
        self
    }
    pub fn with_position<T: Into<Vec3>>(&mut self, p: T) {
        self.with_loc(p.into());
    }
    pub fn with_random_area<T: Into<(Vec3, Vec3)>>(&mut self, p: T) {
        self.with_loc(p.into());
    }

    pub fn complete(&mut self) -> &mut Self {
        let mut l = Living::default();

        if self.race.is_empty() {
            // random race
        }

        self
    }
}
impl Into<Living> for SpawnRequest {
    fn into(self) -> Living {
        Living{
            id: self.id.to_string(),
            attrs: Default::default(),
            has_job: 0,
            // birthday: 0,
            // traits: vec![],
            position: self.location.unwrap().t.clone().unwrap(),
            species: "".to_string()
        }
    }
}

impl<'w, 's> SpawnLiving<'w, 's> for Commands<'w, 's> {
    fn spawn_living<'a>(&'a mut self, sr: SpawnRequest) -> EntityCommands<'w, 's, 'a> {
        let mut base = self.spawn();
        base.insert(Living::from(sr.into()));
        // let mut base = self.spawn_bundle(LivingBundle {
        //     // living: ld.clone(),
        //     // transform: Transform::from_translation(ld.position.clone()),
        //     // global_transform: GlobalTransform::default(),
        // });
        base.with_children(|cmds| {
            // cmds.spawn_scene(srv.get_handle());
        });
        base
    }
}

pub struct Owned(bool);

impl Living {
    pub fn active_job(self) -> i64 {
        self.has_job
    }
}

type Scorer<T = f32> = (T, T, T, T, T, T);
pub trait DerivedAttributes {
    fn mining(self) -> f32;
    fn hauling(self) -> f32;
    fn building(self) -> f32;
    fn score(self, weights: Scorer) -> f32;
}

impl DerivedAttributes for CoreAttributes {
    fn mining(self) -> f32 {
        self.score((35., 10., 10., 35., 5., 5.))
    }
    fn hauling(self) -> f32 {
        self.score((35., 20., 10., 35., 1., 1.))
    }
    fn building(self) -> f32 {
        self.score((20., 15., 25., 15., 1., 25.))
    }

    fn score(self, weights: Scorer) -> f32 {
        (self.constitution * weights.0/100.) +
            (self.agility * weights.1/100.) +
            (self.dexterity * weights.2/100.) +
            (self.strength * weights.3/100.) +
            (self.luck * weights.4/100.) +
            (self.intelligence * weights.5/100.)
    }
}

/*
what is strength

The word ‘strength’ is in many ways an illusion. It can hide us from our vulnerabilities and insecurities. In this blog series, I will look at the different ways that people use the word ‘strength’, and the consequences of our interpretations.

The strength I have come to think of as my personal strength and is the ability to be honest with myself and be honest with others, the ability to see the reality of my life and not to pretend that I am someone I am not.

The word ‘strength’ comes from the Latin word ‘strenuus


what is strength? what is dexterity? what is agility? what is
knowledge? what is wisdom? what is beauty?

What is a friend? what is love? what is a
woman? what is a man? what is a man's wife?
what is a friend's wife? what is a wife's husband?
what is a husband's wife? what is a
husband's friend? what is a wife's friend?

What is heaven? what is hell? what is the
soul? what is the body? what is the world? what
is death? what is life? what is sin? what is virtue?
what is a


 */
