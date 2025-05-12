use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy_bvh_anim::prelude::*;
use bevy_egui::egui;

use crate::bvh_manager::bvh_library::BvhLibrary;
use crate::bvh_manager::bvh_player::{BvhPlayer, SelectedBvhAsset};
use crate::scene_loader::{GroundPlane, MainScene};
use crate::{GameMode, LARGE_EPSILON};

use super::groupbox;

pub fn config_panel(ui: &mut egui::Ui, world: &mut World) {
    ui.heading("Configurations");
    ui.add_space(10.0);
    bvh_map_label(ui, world);
    bvh_playback(ui, world);
    bvh_trail_config(ui, world);
    show_character_checkbox(ui, world);
    draw_main_armature_checkbox(ui, world);
    draw_trajectory_checkbox(ui, world);
    show_ground_checkbox(ui, world);
}

fn bvh_playback(ui: &mut egui::Ui, world: &mut World) {
    let mut params = SystemState::<(
        Res<AssetServer>,
        Res<Assets<BvhAsset>>,
        ResMut<SelectedBvhAsset>,
        ResMut<BvhPlayer>,
        Res<State<GameMode>>,
        ResMut<NextState<GameMode>>,
    )>::new(world);
    let (
        asset_server,
        bvh_assets,
        mut selected_bvh_asset,
        mut bvh_player,
        game_mode,
        mut next_game_mode,
    ) = params.get_mut(world);

    groupbox(ui, |ui| {
        // Choose Bvh file
        ui.horizontal(|ui| {
            ui.label("Choose Bvh File:");

            let mut selected_name = String::new();
            if let Some(path) = asset_server.get_path(selected_bvh_asset.0) {
                selected_name = path.to_string();
            }

            egui::ComboBox::from_label("")
                .selected_text(selected_name)
                .show_ui(ui, |ui| {
                    for id in bvh_assets.ids() {
                        let Some(bvh_name) = asset_server.get_path(id) else {
                            continue;
                        };
                        if ui.selectable_label(false, bvh_name.to_string()).clicked() {
                            selected_bvh_asset.0 = id;
                            if let Some(bvh) = bvh_assets.get(id).map(|asset| &**asset) {
                                bvh_player.duration = bvh.frame_time().as_secs_f32()
                                    * bvh.num_frames().saturating_sub(1) as f32;
                            }
                        }
                    }
                });
        });

        // Playback Ui
        ui.horizontal(|ui| {
            let button_text = match **game_mode {
                GameMode::Config => "Pause",
                _ => "Play",
            };

            if ui.button(button_text).clicked() {
                match **game_mode {
                    GameMode::Config => next_game_mode.set(GameMode::None),
                    _ => next_game_mode.set(GameMode::Config),
                }
            }

            let playback_duration = bvh_player.duration;
            ui.add(egui::Slider::new(
                &mut bvh_player.current_time,
                0.0..=playback_duration - LARGE_EPSILON,
            ));
        });
    });
}

fn bvh_map_label(ui: &mut egui::Ui, world: &mut World) {
    let bvh_library = world.resource::<BvhLibrary>();
    ui.horizontal(|ui| {
        ui.label("Bvh Map: ");
        if let Some(map_path) = bvh_library.get_map().and_then(|m| m.path()) {
            ui.label(map_path.to_string());
        }
    });
}

fn bvh_trail_config(ui: &mut egui::Ui, world: &mut World) {
    let mut config = world.resource_mut::<BvhTrailConfig>();
    groupbox(ui, |ui| {
        ui.label("Bvh Trail");
        ui.checkbox(&mut config.draw_armatures, "Armatures");
        ui.checkbox(&mut config.draw_trajectory, "Trajectory");
        ui.add(
            egui::Slider::new(
                &mut config.interval,
                BvhTrailConfig::MIN_INTERVAL..=BvhTrailConfig::MAX_INTERVAL,
            )
            .text("Interval"),
        );
        ui.label(format!("Average Velocity: {}", config.average_velocity));
    })
}

fn show_character_checkbox(ui: &mut egui::Ui, world: &mut World) {
    let mut q_main_scene = world.query_filtered::<&mut Visibility, With<MainScene>>();
    let Ok(mut main_scene_vis) = q_main_scene.single_mut(world) else {
        return;
    };

    let mut is_main_scene_visible = matches!(*main_scene_vis, Visibility::Hidden) == false;
    ui.checkbox(&mut is_main_scene_visible, "Show Character");
    match is_main_scene_visible {
        true => *main_scene_vis = Visibility::Inherited,
        false => *main_scene_vis = Visibility::Hidden,
    }
}

fn draw_main_armature_checkbox(ui: &mut egui::Ui, world: &mut World) {
    let mut draw_main_armature = world.resource_mut::<DrawMainArmature>();
    ui.checkbox(&mut draw_main_armature, "Show Main Armature");
}

fn draw_trajectory_checkbox(ui: &mut egui::Ui, world: &mut World) {
    let mut draw_trajectory = world.resource_mut::<DrawTrajectory>();
    ui.checkbox(&mut draw_trajectory, "Show Trajectory Arrows");
}

fn show_ground_checkbox(ui: &mut egui::Ui, world: &mut World) {
    let mut q_ground = world.query_filtered::<&mut Visibility, With<GroundPlane>>();
    let Ok(mut vis) = q_ground.single_mut(world) else {
        return;
    };

    let mut vis_bool = matches!(*vis, Visibility::Hidden) == false;
    ui.checkbox(&mut vis_bool, "Show Ground");
    match vis_bool {
        true => *vis = Visibility::Inherited,
        false => *vis = Visibility::Hidden,
    }
}

#[derive(Resource)]
pub struct BvhTrailConfig {
    pub draw_armatures: bool,
    pub draw_trajectory: bool,
    pub interval: f32,
    pub average_velocity: Vec2,
}

impl BvhTrailConfig {
    pub const MAX_INTERVAL: f32 = 0.3333;
    pub const MIN_INTERVAL: f32 = 0.1667;
}

impl Default for BvhTrailConfig {
    fn default() -> Self {
        Self {
            draw_armatures: true,
            draw_trajectory: true,
            interval: 0.1667,
            average_velocity: Vec2::ZERO,
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct DrawMainArmature(bool);

impl Default for DrawMainArmature {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct DrawTrajectory(bool);

impl Default for DrawTrajectory {
    fn default() -> Self {
        Self(true)
    }
}
