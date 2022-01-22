use bevy::prelude::*;

mod bootstrap;
mod data;
mod fuzz_input;
mod input;
mod math;
mod output;
mod runner;
mod window;

pub use bootstrap::fuzz_bootstrap;
pub use output::{parse_commands, EventOutputPlugin};
pub use runner::fuzz_runner;

pub mod prelude {
    pub use crate::{
        bootstrap::{bin_bootstrap, fuzz_bootstrap, FuzzTarget},
        data::FuzzData,
        fuzz_input::FuzzInput,
        FuzzPlugin,
    };
}

#[derive(Default)]
pub struct FuzzPlugin {}

impl FuzzPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

const CLEAN_STAGE_LABEL: &str = "fuzz_clean_startup_system";

impl Plugin for FuzzPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(|_| {
            panic!("use bevy_fuzz::runner::fuzz_runner(&mut app) instead");
        })
        .add_startup_stage_before(
            StartupStage::PreStartup,
            CLEAN_STAGE_LABEL,
            SystemStage::parallel(),
        )
        .add_startup_system_to_stage(CLEAN_STAGE_LABEL, clean_all_system);
    }
}

fn clean_all_system(mut commands: Commands, all_entities: Query<Entity>) {
    for entity in all_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
