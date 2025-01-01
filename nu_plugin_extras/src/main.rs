mod commands;

use commands::{str_match, str_replacer};
use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand};

pub struct ExtrasPlugin;

impl Plugin for ExtrasPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(str_replacer::StrReplacer),
            Box::new(str_match::StrMatch),
        ]
    }
}

fn main() {
    serve_plugin(&ExtrasPlugin, MsgPackSerializer);
}
