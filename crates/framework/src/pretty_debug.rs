use ur_ecs::resource::Resource;
pub use kira_framework_proc::AsPersistentString;

pub trait KiraPrettyDebug {
    fn pretty_debug(&self) -> String;
}

#[derive(Resource, Debug, Copy, Clone)]
pub struct KiraPrettyDebugToggle(pub bool);