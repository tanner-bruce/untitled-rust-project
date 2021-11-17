use bevy::app::App;
use bevy::ecs::entity::Entity;
use bevy::prelude::*;
use bevy_egui::{
    egui::{CtxRef, Id},
    egui,
};
use crate::AppState;
use crate::ui::menus::{main_menu, new_game};
use crate::sim::world::entity::Living;
use bevy::utils::HashMap;

pub mod research;
pub mod settings;
pub mod menus;
pub mod createchar;

#[derive(Copy, Clone, Debug)]
pub enum UIKind {
    Character,
}

#[derive(Clone, Debug)]
pub struct VisibleUI {
    name: UIKind,
    pub close: bool,
}

impl VisibleUI {
    pub fn new(uk: UIKind) -> Self {
        VisibleUI {
            name: uk,
            close: true,
        }
    }

    pub fn draw(&mut self, e: Entity, l: &Living, ctx: &CtxRef) {
        match self.name {
            UIKind::Character => {
                egui::Window::new("Character")
                    .open(&mut self.close)
                    .id(Id::new(e.id()+1))
                    .show(ctx, |ui| {
                        ui.vertical(|ui| {
                            let attrs = l.attrs;
                            ui.columns(2, |ui| {
                                ui[0].label("Strength");
                                ui[1].label(attrs.strength.to_string());
                            });
                            ui.columns(2, |ui| {
                                ui[0].label("Constitution");
                                ui[1].label(attrs.constitution.to_string());
                            });
                            ui.columns(2, |ui| {
                                ui[0].label("Dexterity");
                                ui[1].label(attrs.dexterity.to_string());
                            });
                            ui.columns(2, |ui| {
                                ui[0].label("Agility");
                                ui[1].label(attrs.agility.to_string());
                            });
                            ui.columns(2, |ui| {
                                ui[0].label("Intelligence");
                                ui[1].label(attrs.intelligence.to_string());
                            });
                            ui.columns(2, |ui| {
                                ui[0].label("Luck");
                                ui[1].label(attrs.luck.to_string());
                            });
                        })
                    });
            }
        }
    }
}

pub struct System;

impl Plugin for System {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(menus::main_menu))
            .add_system_set(SystemSet::on_update(AppState::NewGame).with_system(menus::new_game))
            // .add_system_set(SystemSet::on_update(GameState::Loading).with_system(loading.system()))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(menus::in_game));
    }
}