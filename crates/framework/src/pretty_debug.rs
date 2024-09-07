use bevy_ecs::system::Resource;

pub trait KiraPrettyDebug {
    fn pretty_debug(&self) -> String;
}

#[derive(Resource, Debug, Copy, Clone)]
pub struct KiraPrettyDebugToggle(pub bool);