use bevy::prelude::*;
use bevy_motion_matching::{
    record,
    trajectory::{self, Velocity},
};

mod bvh_gizmos;
mod camera;
mod player;
mod scene_loader;
mod ui;
mod visualization;

fn main() {
    App::new()
        .add_plugins((
            bevy_motion_matching::MotionMatchingAppPlugin,
            bvh_gizmos::BvhGizmosPlugin,
            record::RecordPlugin::<Velocity>::default(),
            trajectory::TrajectoryPlugin,
            visualization::VisualizationPlugin,
            scene_loader::SceneLoaderPlugin,
            player::PlayerPlugin,
            camera::CameraPlugin,
            ui::UiPlugin,
        ))
        .run();
}
