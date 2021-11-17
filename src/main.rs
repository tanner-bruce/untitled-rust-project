#![feature(derive_default_enum)]

use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Add, DerefMut, RangeInclusive},
};

// use bevy_asset_loader::AssetLoader;
use bevy::{
    pbr::PbrPlugin,
    gltf::{Gltf, GltfMesh, GltfPrimitive},
    app::AppExit,
    core::{FixedTimestep, FixedTimesteps},
    ecs::{
        component::Component,
        query::FilterFetch,
    },
    input::{
        ElementState,
        InputPlugin,
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion},
    },
    pbr::{AmbientLight, PointLightBundle},
    prelude::*,
    utils::HashMap,
    window::WindowId,
    ecs::event::Events,
};
use bevy_egui::{
    egui::{
        self,
        Align2,
        Area,
        CtxRef,
        emath,
        Frame,
        Id,
        LayerId,
        Pos2,
        TopBottomPanel,
        Widget,
        Window,
    },
    EguiContext,
    EguiPlugin,
    WindowSize,
};
use rand::Rng;
use serde_yaml::to_string;

use crate::{
    sim::generator::WorldOptions,
    sim::world::entity::{CoreAttributes, Living},
    sim::world::buildings,
    sim::world::data::*,
    sim::orders::{Orders, Build, BuildOrder, InputOrder, MakeOrder}
};
use bevy_event_set::*;
use crate::sim::world::entity::{SpawnLiving, SpawnRequest};
use bevy::utils::{StableHashSet, StableHashMap};
use bevy::asset::{HandleId, LoadState};
use bevy::render::camera::{CameraProjection, OrthographicProjection};
use bevy_mod_raycast::*;

pub mod ui;
pub mod input;
pub mod sim;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    NewGame,
    WorldLoad,
}

impl Default for AppState {
    fn default() -> Self {
        AppState::MainMenu
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Loading,
    Running,
    Paused,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Paused
    }
}

const LABEL: &str = "my_fixed_timestep";

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

type MousePosition = Vec2;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum LoadingState {
    Loading,
    Done,
}

struct MyRaycastSet;

fn main() {
    let mut app = App::new();
    // AssetLoader::new(LoadingState::Loading, LoadingState::Done)
    //     // .with_collection::<AudioAssets>()
    //     .build(&mut app);
    app.insert_resource(WindowDescriptor {
        width: 1280.,
        height: 720.,
        title: "Machanofence".to_string(),
        ..Default::default()
    })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
            ..Default::default()
        })
        .insert_resource(
            bevy::log::LogSettings {
                level: bevy::log::Level::from(bevy::log::Level::DEBUG),
                ..Default::default()
            })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_state(LoadingState::Loading)
        .add_state(AppState::MainMenu)
        .add_state(GameState::Paused)
        .add_startup_system(setup)
        .add_startup_system(sys)
        .add_system(render)
        .init_resource::<Schedule>()
        .init_resource::<AppState>()
        .init_resource::<GameState>()
        .init_resource::<StartupData>()
        .init_resource::<TickCount>()
        .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
        .add_system_to_stage(
            CoreStage::PreUpdate,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays),
        )
        .add_plugin(sim::System)
        .add_plugin(input::System)
        // .add_system_set(SystemSet::on_update(AppState::InGame).with_system(game_running.system()))
        .add_plugin(ui::System)
        // .add_system(SystemSet::on_enter(AppState::InGame).with_system(ingame.system()))
        // add a new stage that runs every two seconds
        .add_stage_after(
            CoreStage::PreUpdate,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(
                    FixedTimestep::step(0.01)
                        // labels are optional. they provide a way to access the current
                        // FixedTimestep state from within a system
                        .with_label(LABEL),
                )
                .with_system(queue_checker),
        ).run();
}

fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<MyRaycastSet>>,
) {
    for mut pick_source in &mut query.iter_mut() {
        if let Some(cursor_latest) = cursor.iter().last() {
            pick_source.cast_method = RayCastMethod::Screenspace(cursor_latest.position);
        }
    }
}

macro_rules! s {
    ($name:expr) => {
        String::from($name)
    };
}
#[derive(Debug, Default)]
pub struct StartupData {
    spawned: bool,
    opts: WorldOptions,
}
#[derive(Default)]
pub struct TickCount(i64);

fn queue_checker(
    mut w: ResMut<World>,
    mut s: ResMut<Schedule>,
    mut tc: ResMut<TickCount>,
) {
    tc.0 += 1;
    send_order::<BuildOrder, Build>(&mut w, Build{
        origin: (0.0, 0.0),
        building_id: 0,
        rotation: 0,
        ghost: false
    });
    s.run_once(&mut w);
}

fn send_order<T: 'static, D>(mut w: &mut World, data: D)
    where
        T: MakeOrder<D> + Sync + Send,
        D: Send + Sync {
    let mut ords = w.get_resource_mut::<Events<T>>();
    if !ords.is_none() {
        let d = T::new(data);
        ords.unwrap().send(d);
    }
}


// called to setup the game struct
fn setup(mut commands: Commands,
         asset_server: Res<AssetServer>,
         mut sched: ResMut<Schedule>,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // load all the entity data resources
    commands.insert_resource(EntityData::new(&asset_server));

    // spawn the camera
    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_xyz(40., 15., 1.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    //     ..Default::default()
    // });
    let mut bundle = OrthographicCameraBundle::new_3d();
    bundle.orthographic_projection.scale = 32.;
    bundle.transform = Transform::from_xyz(120., 45., 0.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);

    commands.spawn_bundle(bundle).insert(RayCastSource::<MyRaycastSet>::new_transform_empty());;

    // create some light
    commands
        .spawn_bundle(
        PointLightBundle {
            point_light: PointLight{
                color: Default::default(),
                intensity: 500.0,
                range: 500.0,
                radius: 50.0
            },
            transform: Transform::from_xyz(5.0, 15.0, 3.0),
            ..Default::default()
        }
    );

    // daylight
    commands.spawn().insert(
        DirectionalLight::new(
            Default::default(),
            50000.,
            Transform::from_xyz(500., 500., 0.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y).rotation.xyz()
        ));


    let mut inner = App::new();
    inner.add_startup_system(inner_start).add_system(|mut q: Query<(&mut Living)>| {
        for l in q.iter_mut() {
            let l1 = l.into_inner();
            l1.position.x += 0.01;
            l1.position.z += 0.01;
        }
    });
    let mut sim = sim::Sim::new(&mut inner.world, WorldOptions::default());
    let added = inner.world.query_filtered::<(Entity, &Living), Added<Living>>();

    sched.add_stage("first", SystemStage::parallel());

    commands.insert_resource(inner.world);
    commands.insert_resource(inner.schedule);
    commands.insert_resource(added);

    commands.spawn_bundle(
        (
            Transform::default(),
            GlobalTransform::identity(),
        )
    ).with_children(|p| {
        p.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        }).insert(RayCastMesh::<MyRaycastSet>::default());
    });
}


struct SimEnt(u32);

fn inner_start(mut commands: Commands) {
    commands.spawn_living(SpawnRequest{
        id: s!("sphere_worker"),
        race: s!("Sphere"),
        location: Some(Vec3::new(0., 0., 0.).into()),
        name: Some(s!("Yo dude")),
        birthday: None,
        traits: None,
        tags: None,
        living: None
    });
    commands.spawn_living(SpawnRequest{
        id: s!("birch_tree"),
        race: s!("Sphere"),
        location: Some(Vec3::new(5., 0., 0.).into()),
        name: Some(s!("Yo dude")),
        birthday: None,
        traits: None,
        tags: None,
        living: None
    });
}

#[derive(Debug)]
pub struct TrackedEntity(u32);

pub fn render(
    mut commands: Commands,
    mut ed: ResMut<EntityData>,
    mut w: ResMut<World>,
    mut q: Query<(&mut Transform, &TrackedEntity)>,
    ass: Res<AssetServer>,
) {
    // transform the iterator into a hashmap
    let mut transforms: StableHashMap<u32, Mut<Transform>> = Default::default();
    for (transform, tracked) in q.iter_mut() {
        transforms.insert(tracked.0, transform);
    }

    // iterate the sim entities, check hashmap for existence, otherwise render
    for (e, l) in w.query::<(Entity, &Living)>().iter(&w) {
        let e: Entity = e;
        let l: &Living = l;
        let o = transforms.get_mut(&e.id());
        match o {
            Some(t) => {
                (*t).translation = l.position;
            },
            None => {
                let hid = ed.data.get(l.id.as_str());
                match ass.get_load_state(ass.get_handle::<Scene, HandleId>(hid.unwrap().clone())) {
                    LoadState::Loaded => {
                        info!("my pos {:?}", l.position);
                        commands.spawn_bundle(
                                (
                                    Transform::from_translation(l.position),
                                    GlobalTransform::identity(),
                                    TrackedEntity(e.id()),
                                    RayCastMesh::<MyRaycastSet>::default()
                                )
                            ).with_children(|b| {
                                b.spawn_scene(ass.load(Entities.get(l.id.as_str()).unwrap().scene().as_str()));
                            });
                    },
                    _ => {

                    }
                }
            }
        }
    }

}



fn sys(mut commands: Commands, mut s: ResMut<StartupData>, ass: Res<AssetServer>) {
    if s.spawned {
        return;
    }

    for x in 1..20 {
        let mut r = rand::thread_rng();
        let mut x: f32 = r.gen_range(-15.0..=15.0);
        let mut y: f32 = r.gen_range(-15.0..=15.0);
        commands.spawn_bundle((
            Transform::from_xyz(x, 0.0, y),
            GlobalTransform::identity()
        )).with_children(|parent| {
            // parent.spawn_scene(ass.get_handle(ed.building_handle("wall")));
        });
    }
    s.spawned = true;
}