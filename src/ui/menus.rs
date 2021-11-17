use crate::{
    StartupData,
    AppState,
    TickCount,
    ui::VisibleUI,
    input::InteractionContext,
    sim::{
        world::{
            data::Buildings,
            entity::Living
        }
    }
};
use bevy_egui::{
    EguiContext,
    egui,
    egui::{Widget, Frame},
    egui::{Align2, TopBottomPanel}
};
use bevy::prelude::*;
use std::ops::RangeInclusive;

pub fn main_menu(egui_context: ResMut<EguiContext>, tc: Res<TickCount>, mut state: ResMut<State<AppState>>) {
    let w = egui::Window::new("Mechanomancer")
        .anchor(Align2::CENTER_CENTER, [0., 0.]);

    w.show(egui_context.ctx(), |ui| {
        ui.label(tc.0.to_string());
        let ng = ui.button("New Game");
        let lg = ui.button("Load Game");
        let stt = ui.button("Settings");
        let x = ui.button("Exit");

        if ng.clicked() {
            state.set(AppState::NewGame).unwrap()
        }
        if x.clicked() {
            std::process::exit(0);
        }
    });
}

pub fn new_game(egui_context: ResMut<EguiContext>, mut ngs: ResMut<StartupData>, mut state: ResMut<State<AppState>>) {
    bevy_egui::egui::Window::new("New Game")
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .show(egui_context.ctx(), |ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.columns(2, |ui| {
                            ui[0].label("Seed");
                            ui[1].text_edit_singleline(&mut ngs.opts.seed);
                        });
                    });
                    let mo = &mut ngs.opts;
                    ui.collapsing("Environment",|ui| {
                        for (n, p, mn, mx) in [
                            ("Temperature", &mut mo.temp_avg, -32.0, 32.0),
                            ("Resource Richness", &mut mo.rsrc_rich, -50.0, 300.0),
                            ("Resource Abundance", &mut mo.rssc_abund, -50.0, 300.0),
                            ("Water", &mut mo.water, -100.0, 300.0),
                            ("Flatness", &mut mo.flatness, 0.0, 100.0),
                            ("Vegetation", &mut mo.vegetation, -100.0, 300.0)
                        ] {
                            ui.horizontal(|ui| {
                                ui.columns(2, |ui| {
                                    ui[0].label(n);
                                    egui::Slider::new(p, RangeInclusive::new(mn, mx)).ui(&mut ui[1]);
                                });
                            });
                        }
                    });

                    ui.collapsing("Hostility",|ui| {
                        for (n, p, mn, mx, dsc) in [
                            ("Proclensity", &mut mo.proclensity, -100.0, 100.0, "How good or evil a location is"),
                            ("Hostile Density", &mut mo.hostile_density, 0.0, 600.0, "Density of hostiles"),
                            ("Hostile Evolution", &mut mo.hostile_evolution, 0.0, 3.0, "How frequently hostiles evolve"),
                            ("Hostile Intelligence", &mut mo.hostile_intelligence, 1.0, 3.0, "How smart hostiles will fight"),
                            ("Hostile Drop Rate", &mut mo.hostile_drop_rate, 1.0, 4.0, "The amount of loot hostiles drop"),
                            ("Darkness Density", &mut mo.dark_density, 0.1, 4.0, "How dense the darkness is"),
                            ("Darkness Spread", &mut mo.dark_spread, 0.1, 4.0, "How quickly the darkness will spread"),
                            ("Pollution Spread", &mut mo.pollution_spread, 0.1, 4.0, "How much your pollution feeds the darkness"),
                            ("Heat Spread", &mut mo.heat_spread, 0.1, 4.0, "How much your head feeds the darkness")
                        ] {
                            let h = ui.horizontal(|ui| {
                                let cs = ui.columns(2, |ui| {
                                    ui[0].label(n);
                                    egui::Slider::new(p, RangeInclusive::new(mn, mx)).ui(&mut ui[1]);
                                });
                            });
                        }
                    });
                });
            }); // world stats
            ui.group(|ui| {
                ui.heading("Character");
            }); // character stats

            // forward, back
            ui.separator();
            TopBottomPanel::bottom("ng_ctrl").show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    let back = ui.button("Back");
                    let start = ui.button("Start");
                    if back.clicked() {
                        state.set(AppState::MainMenu).unwrap();
                    }
                    // TODO: verification
                    if start.clicked() {
                        state.set(AppState::InGame).unwrap();
                    }
                });
            }); // bottom bar
        });
}

pub fn in_game(egui_context: ResMut<EguiContext>,
               mut g: ResMut<InteractionContext>,
               // mut ed: ResMut<EntityData>,
               mut q: Query<(
                   Entity,
                   Option<&mut VisibleUI>, Option<&Living>)
               >) {
    // draw all visible ui components
    q.for_each_mut(|(e, o, l)| {
        match o {
            Some(mut o) => {
                o.draw(e, l.unwrap(), egui_context.ctx())
            },
            None => (),
        }
    });
    if g.build_mode {
        // todo: build this from data table
        let w = bevy_egui::egui::Window::new("Build")
            .open(&mut g.build_mode)
            .anchor(Align2::RIGHT_CENTER, [0., 0.])
            .show(egui_context.ctx(), |ui| {
                ui.vertical(|ui| {
                    Buildings.iter().for_each(|(k, v)| {
                        ui.button(v.name.clone());

                    });
                    // ui.collapsing("Defense", |ui| {
                    //     ui.horizontal(|ui| {
                    //         if ui.button("Road").clicked() {
                    //         }
                    //         ui.button("Wall");
                    //         ui.button("Turret");
                    //         ui.button("Floodlight");
                    //     });
                    // });
                    // ui.collapsing("Production", |ui| {
                    //     ui.horizontal(|ui| {
                    //         if ui.button("Factory").clicked() {
                    //         }
                    //         ui.button("Smelter");
                    //         ui.button("Conveyor");
                    //         ui.button("Loader");
                    //     });
                    // });
                    // ui.collapsing("Storage", |ui| {
                    //     ui.horizontal(|ui| {
                    //         if ui.button("Box").clicked() {
                    //         }
                    //         ui.button("Conveyor");
                    //     });
                    // });
                });
            });
    } // build menu
    // toolbar
    bevy_egui::egui::Window::new("toolbelt")
        .title_bar(false)
        .resizable(false)
        .frame(Frame::default().corner_radius(1.0))
        .anchor(Align2::CENTER_BOTTOM, [0., -3.])
        .frame(Frame::group(&bevy_egui::egui::Style::default()))
        .show(egui_context.ctx(), |ui|{
            ui.horizontal(|ui| {
                ui.button("tb0");
                ui.button("tb1");
                ui.button("tb2");
                ui.button("tb3");
                ui.button("tb4");
                ui.separator();
                ui.button("tb5");
                ui.button("tb6");
                ui.button("tb7");
                ui.button("tb8");
                ui.button("tb9");
            })
        }); // toolbar
    if g.show_inventory {
        egui::Window::new("Player Inventory")
            .open(&mut g.show_inventory)
            .show(egui_context.ctx(), |ui| {
                // inventory slots and detail view
                ui.columns(2, |uis| {
                    uis[0].vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.button("[slot]");
                            ui.button("[slot]");
                            ui.button("[slot]");
                            ui.button("[slot]");
                            ui.button("[slot]");
                        });
                    }); // slots
                    uis[1].group(|ui| {
                        ui.label("lbl")
                    })
                });
            });
    } // inventory
}
