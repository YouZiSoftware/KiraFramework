use std::ffi::OsStr;
use bevy_app::{App, Plugin};
use libloading::Library;

pub struct LibPlugin {
    _lib: Library,
    plugin: Box<dyn Plugin>,
}

impl LibPlugin {
    pub fn new<P: AsRef<OsStr>>(path: P) -> anyhow::Result<Self> {
        unsafe {
            let lib = Library::new(
                path
            )?;
            let func: libloading::Symbol<fn() -> *mut dyn Plugin> =
                lib.get(b"kira_framework_get_plugin")?;
            let x = func();
            Ok(Self {
                _lib: lib,
                plugin: Box::from_raw(x),
            })
        }
    }
}

impl Plugin for LibPlugin {
    fn build(&self, app: &mut App) {
        self.plugin.build(app);
    }

    fn ready(&self, app: &App) -> bool {
        self.plugin.ready(app)
    }

    fn finish(&self, app: &mut App) {
        self.plugin.finish(app);
    }

    fn cleanup(&self, app: &mut App) {
        self.plugin.cleanup(app);
    }

    fn name(&self) -> &str {
        self.plugin.name()
    }

    fn is_unique(&self) -> bool {
        self.plugin.is_unique()
    }
}