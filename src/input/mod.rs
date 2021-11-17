use bevy::{
    prelude::*,
    input::ElementState,
    input::keyboard::KeyboardInput,
    input::mouse::{MouseButtonInput, MouseMotion},
    utils::StableHashMap
};
use bevy_egui::{
    egui::Context,
    EguiContext
};
use bevy_event_set::*;
use crate::{
    MousePosition,
    sim::orders::*,
};
use std::{
    collections::HashMap,
    sync::Arc
};
use bevy::math::{Vec3Swizzles, Vec4Swizzles};
use bevy::render::camera::{OrthographicProjection, CameraProjection};
use std::ops::Sub;

pub struct System;

impl Plugin for System {
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractionContext>()
            .add_system(capture_keyboard_events)
            .add_system(capture_mouse_clicks)
            .add_system(capture_mouse_position);
    }
}

#[derive(Default, Clone)]
pub struct InteractionContext {
    pub order_on_cursor: Option<i32>,
    pub build_mode: bool,
    pub selection: Option<Vec<i32>>,
    pub rotation: u8,

    pub mouse_position: (f32, f32),
    pub is_dragging: bool,
    pub is_shift: bool,
    pub is_alt: bool,

    pub show_inventory: bool,
}


// capture_mouse_position captures the current position of the mouse
fn capture_mouse_position(mut mp: ResMut<InteractionContext>, mut events: EventReader<CursorMoved>) {
    let w = events.iter().last();
    match w {
        Some(x) => {
            mp.mouse_position = (x.position.x, x.position.y);
        },
        None => ()
    }
}

fn capture_keyboard_events(
    mut ctx: ResMut<EguiContext>,
    mut orders: Orders,
    mut ic: ResMut<InteractionContext>,
    mut kb_events: EventReader<KeyboardInput>) {
    // don't handle keyboard input while UI is using
    // todo: some keycodes should still be handled
    if ctx.ctx().wants_keyboard_input() {
        return;
    }

    for event in kb_events.iter() {
        match event.key_code {
            Some(v) => {
                match v {
                    KeyCode::LShift => {
                        ic.is_shift = event.state == ElementState::Pressed;
                    }
                    KeyCode::RShift => {
                        ic.is_shift = event.state == ElementState::Pressed;
                    }
                    KeyCode::Escape => {
                        // g.selection = 0;
                    },
                    KeyCode::I => {
                        if event.state == ElementState::Pressed {
                            // g.show_inventory = !g.show_inventory;
                        }
                    },
                    KeyCode::C => {
                        if event.state == ElementState::Pressed {
                            // g.show_character = !g.show_character;
                        }
                    },
                    KeyCode::B => {
                        if event.state == ElementState::Pressed {
                            ic.build_mode = !ic.build_mode;
                            orders.send(BuildOrder::new(Build{
                                origin: (0.0, 0.0),
                                building_id: 0,
                                rotation: 0,
                                ghost: false
                            }));
                        }
                    },
                    KeyCode::Tab => {
                        if event.state == ElementState::Pressed {
                            // g.show_crafting = !g.show_crafting;
                        }
                    },
                    KeyCode::Space => {
                        if event.state == ElementState::Pressed {
                            // g.paused = !g.paused;
                        }
                    }
                    _ => {}
                }
            },
            None => (),
        }
    }
}

fn capture_mouse_clicks(
    mut commands: Commands,
    ass: Res<AssetServer>,
    windows: Res<Windows>,
    mut ic: ResMut<InteractionContext>,
    mut orth: Query<(&OrthographicProjection, &Transform)>,
    mut mouse_input_events: EventReader<MouseButtonInput>,
) {
    let opt = orth.iter().last();
    let (e, q): (&OrthographicProjection, &Transform) = opt.unwrap();
    let inv_proj = (*e).get_projection_matrix().inverse();
    let camera_position = q.compute_matrix();
    let ndc_to_world: Mat4 = camera_position * inv_proj;
    let window = windows.get_primary().unwrap();
    let ws = Vec2::from([window.width(), window.height()]);
    for event in mouse_input_events.iter() {
        let ev: &MouseButtonInput = event;
        if ev.state == ElementState::Pressed {
            continue;
        }
        /*
        float x = (2.0f * mouse_x) / width - 1.0f;
    float y = 1.0f - (2.0f * mouse_y) / height;
    float z = 1.0f;
    vec3 ray_nds = vec3(x, y, z);
         */
        let ipos = ic.mouse_position;
        let pos = Vec2::from([ipos.0, ipos.1]);
        let ndc = (pos / ws) * 2. - Vec2::from([-1., -1.]);
        let cursor_pos_ndc_near: Vec3 = ndc.extend(-1.0);
        let cursor_pos_ndc_far: Vec3 = ndc.extend(1.0);
        let cursor_pos_near: Vec3 = ndc_to_world.project_point3(cursor_pos_ndc_near);
        let cursor_pos_far: Vec3 = ndc_to_world.project_point3(cursor_pos_ndc_far);
        let rw = cursor_pos_far - cursor_pos_near;

        info!("{:?}", rw);


        let norm = Vec3::from([0., 1., 0.]);
        let denom = norm.dot(rw);
        if denom.abs() > 0.0001 {
            // let i = q.translation.sub(rw);
            let t = q.translation.sub(rw).dot(norm) / denom;
            if t >= 0. {
                let t = (-(q.translation.dot(norm) + 0.000001)) / rw.dot(norm);
                let p = q.translation + t*rw;
                info!("{:?}", p);

                commands.spawn_bundle(
                    (
                        Transform::from_translation(p),
                        GlobalTransform::identity(),
                    )
                ).with_children(|c|{
                    c.spawn_scene(ass.load("sphere.gltf#Scene0"));
                });

                info!("hit!");
            } else {
                info!("no hit!");
            }
        }
        info!("{:?}", event);
        return;
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct KeymapEntry {
    kc: Option<KeyCode>,
    mb: Option<MouseButton>
}

pub trait IntoCode {
    fn into(self) -> KeymapEntry;
}

impl IntoCode for KeyCode {
    fn into(self) -> KeymapEntry {
        KeymapEntry{
            kc: Some(self),
            mb: None,
        }
    }
}

impl IntoCode for MouseButton {
    fn into(self) -> KeymapEntry {
        KeymapEntry{
            mb: Some(self),
            kc: None,
        }
    }
}


