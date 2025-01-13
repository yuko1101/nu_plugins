use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{record, LabeledError, Signature, Span, Type, Value};
use scraper::{ElementRef, Html, Node};

use crate::ExtraParsersPlugin;

pub struct FromHtml;

impl SimplePluginCommand for FromHtml {
    type Plugin = ExtraParsersPlugin;

    fn name(&self) -> &str {
        "from html"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::String, Type::record())
            .switch("fragment", "parse as a fragment", Some('f'))
    }

    fn description(&self) -> &str {
        "Convert HTML to a record"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let input_span = input.span();
        let input = match input {
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

        let is_fragment = call.has_flag("fragment")?;

        let parsed = if is_fragment {
            Html::parse_fragment(input)
        } else {
            Html::parse_document(input)
        };

        let record = html_to_record(parsed.root_element(), input_span);

        Ok(record)
    }
}

fn html_to_record(element_ref: ElementRef, span: Span) -> Value {
    let mut record = record!();
    let element = element_ref.value();

    let mut attrs_record = record!();
    for (key, value) in element.attrs() {
        attrs_record.insert(key, Value::string(value, span));
    }
    record.insert("attrs", Value::record(attrs_record, span));

    let children = element_ref.children();
    let mut children_list = vec![];
    for node_ref in children {
        match node_ref.value() {
            Node::Element(_) => {
                children_list.push(html_to_record(ElementRef::wrap(node_ref).unwrap(), span));
            }
            Node::Text(text) => {
                if text.trim().is_empty() {
                    continue;
                }
                children_list.push(Value::string(text.to_string(), span));
            }
            _ => {}
        }
    }
    record.insert("children", Value::list(children_list, span));

    record.insert("tag", Value::string(element.name().to_string(), span));

    Value::record(record, span)
}
