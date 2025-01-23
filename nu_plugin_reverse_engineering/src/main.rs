mod commands;

use commands::mt64::Mt64Command;
use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand};

pub struct ReverseEngineeringPlugin;

impl Plugin for ReverseEngineeringPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Mt64Command)]
    }
}

fn main() {
    serve_plugin(&ReverseEngineeringPlugin, MsgPackSerializer);
}
