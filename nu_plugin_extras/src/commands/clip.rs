use arboard::Clipboard;
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, Signature, Value};

use crate::ExtrasPlugin;

pub struct Clip;

impl SimplePluginCommand for Clip {
    type Plugin = ExtrasPlugin;

    fn name(&self) -> &str {
        "clip"
    }

    fn description(&self) -> &str {
        "Copy the input to the system clipboard."
    }

    fn signature(&self) -> Signature {
        Signature::build("clip")
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let input_text = match input {
            Value::String { val, .. } => val,
            _ => {
                return Err(
                    LabeledError::new("Expected String input from pipeline").with_label(
                        format!("requires string input; got {}", input.get_type()),
                        call.head,
                    ),
                )
            }
        };

        let mut clipboard = Clipboard::new().map_err(|e| {
            LabeledError::new("Failed to access clipboard")
                .with_label(format!("Error: {}", e), call.head)
        })?;

        clipboard.set_text(input_text).map_err(|e| {
            LabeledError::new("Failed to set clipboard text")
                .with_label(format!("Error: {}", e), call.head)
        })?;

        Ok(input.clone())
    }
}


pub struct ClipGet;

impl SimplePluginCommand for ClipGet {
    type Plugin = ExtrasPlugin;

    fn name(&self) -> &str {
        "clip get"
    }

    fn description(&self) -> &str {
        "Get the content of the system clipboard."
    }

    fn signature(&self) -> Signature {
        Signature::build("clip get")
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let mut clipboard = Clipboard::new().map_err(|e| {
            LabeledError::new("Failed to access clipboard")
                .with_label(format!("Error: {}", e), call.head)
        })?;

        let text = clipboard.get_text().map_err(|e| {
            LabeledError::new("Failed to get clipboard text")
                .with_label(format!("Error: {}", e), call.head)
        })?;

        Ok(Value::string(text, call.head))
    }
}