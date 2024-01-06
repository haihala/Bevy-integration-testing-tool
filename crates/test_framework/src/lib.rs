mod input_playback;
pub use input_playback::PlaybackTestGear;

use bevy::{ecs::system::SystemId, prelude::*};

#[derive(Resource, Debug)]
pub struct AssertSystem(pub SystemId);

#[macro_export]
macro_rules! test_scenario {
    ($script_name:expr, $assert_system:ident, $appPlugins:ident) => {
        use bevy::prelude::*;

        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        app.add_plugins(PlaybackTestGear::new($script_name.into()));
        let assert_sys_id = app.world.register_system($assert_system);
        app.insert_resource(AssertSystem(assert_sys_id));

        app.add_plugins($appPlugins).run();
    };
}
