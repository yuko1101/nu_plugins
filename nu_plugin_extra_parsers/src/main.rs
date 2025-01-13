mod html;

use html::from_html::FromHtml;
use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand};

pub struct ExtraParsersPlugin;

impl Plugin for ExtraParsersPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(FromHtml)]
    }
}

fn main() {
    serve_plugin(&ExtraParsersPlugin, MsgPackSerializer);
}
