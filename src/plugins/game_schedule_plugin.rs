use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StartupSet {
    Assets,
    World,
    Entities,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSet {
    Input,
    Spawning,
    Physics,
    Wrap,
    Collision,
    Cleanup,
}

pub struct GameSchedulePlugin;

impl Plugin for GameSchedulePlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(
                Startup,
                (
                    StartupSet::Assets,
                    StartupSet::World,
                    StartupSet::Entities,
                )
                    .chain(),
            )
            .configure_sets(
                Update,
                (
                    GameSet::Input,
                    GameSet::Physics,
                    GameSet::Wrap,
                    GameSet::Collision,
                    GameSet::Cleanup,
                )
                    .chain(),
            );
    }
}