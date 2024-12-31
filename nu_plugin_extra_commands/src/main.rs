mod commands;

use commands::str_replacer;
use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand};

pub struct ExtraCommandsPlugin;

impl Plugin for ExtraCommandsPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(str_replacer::StrReplacer)]
    }
}

fn main() {
    serve_plugin(&ExtraCommandsPlugin, MsgPackSerializer);
}
