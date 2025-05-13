use std::io::Write;

use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy_bvh_anim::prelude::*;
use bevy_egui::egui;
use bevy_platform::collections::{HashMap, HashSet};

use crate::bvh_manager::bvh_library::BvhLibrary;
use crate::motion::motion_asset::MotionAsset;
use crate::motion::trajectory_data::TrajectoryDataConfig;
use crate::trajectory::TrajectoryConfig;

use super::scrollbox;

#[derive(Resource, Default, Debug)]
pub struct BuildConfigs {
    pub selections: HashMap<AssetId<BvhAsset>, bool>,
    pub bvh_assets: HashSet<AssetId<BvhAsset>>,
}

pub struct BuildConfig {
    pub selection: bool,
    pub loopable: bool,
}

pub fn builder_panel(ui: &mut egui::Ui, world: &mut World) {
    ui.heading("Builder");
    ui.add_space(10.0);
    motion_data_asset_buider_menu(ui, world);
    ui.add_space(10.0);
    build_motion_data_asset_button(ui, world);
}

fn motion_data_asset_buider_menu(ui: &mut egui::Ui, world: &mut World) {
    let mut params = SystemState::<(
        Res<AssetServer>,
        Res<Assets<BvhAsset>>,
        ResMut<BuildConfigs>,
    )>::new(world);
    let (asset_server, bvh_assets, mut build_config) = params.get_mut(world);

    ui.label("Bvh Builder");
    ui.add_space(10.0);
    scrollbox(ui, 200.0, |ui| {
        for id in bvh_assets.ids() {
            let Some(bvh_name) = asset_server.get_path(id) else {
                continue;
            };

            let mut is_selected = build_config.bvh_assets.contains(&id);
            if ui
                .checkbox(&mut is_selected, bvh_name.to_string())
                .changed()
            {
                if is_selected {
                    build_config.bvh_assets.insert(id);
                } else {
                    build_config.bvh_assets.remove(&id);
                }
            }
        }
    });
}

fn build_motion_data_asset_button(ui: &mut egui::Ui, world: &mut World) {
    let mut params = SystemState::<(
        Res<BvhLibrary>,
        Res<Assets<BvhAsset>>,
        Res<BuildConfigs>,
        Res<TrajectoryConfig>,
    )>::new(world);
    let (bvh_library, bvh_assets, build_config, trajectory_config) = params.get(world);

    if ui.button("Build").clicked() {
        let Some(bvh_map) = bvh_library
            .get_map()
            .and_then(|handle| bvh_assets.get(handle))
            .map(|asset| &**asset)
        else {
            return;
        };

        let mut motion_data_asset = MotionAsset::new(
            bvh_map,
            TrajectoryDataConfig {
                interval_time: trajectory_config.interval_time,
                num_points: trajectory_config.num_points(),
            },
        );

        motion_data_asset.append_bvhs(
            build_config
                .bvh_assets
                .iter()
                .filter_map(|id| bvh_assets.get(*id)),
        );

        let convert_to_json = serde_json::to_string(&motion_data_asset).unwrap();

        let mut asset_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            // TODO: specify a file name and possibly a location
            .open("assets/motion_data/motion_data.json")
            .unwrap();

        asset_file.write_all(convert_to_json.as_bytes()).unwrap();
    }
}
