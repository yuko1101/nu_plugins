use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{
    ast::RangeInclusion, record, IntRange, LabeledError, Signature, SyntaxShape, Type, Value,
};

use crate::ExtrasPlugin;

use super::str_replacer::match_result_type;

pub struct StrMatch;

impl SimplePluginCommand for StrMatch {
    type Plugin = ExtrasPlugin;

    fn name(&self) -> &str {
        "str match"
    }

    fn description(&self) -> &str {
        "Matches all occurrences of a regex pattern in a string."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::String, Type::list(match_result_type()))
            .required("regex", SyntaxShape::String, "the regex pattern to match")
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

        let regex_string: String = call.req(0)?;
        let re = regex::Regex::new(&regex_string).map_err(|e| {
            LabeledError::new("Invalid regex pattern").with_label(
                format!("Error: {}", e),
                call.positional.get(0).unwrap().span(),
            )
        })?;

        let mut result = vec![];
        for caps in re.captures_iter(input) {
            let mut captures = vec![];
            for cap in caps.iter() {
                if let Some(cap) = cap {
                    captures.push(Value::record(
                        record!(
                            "range" => Value::range(
                                IntRange::new(
                                    Value::int(cap.start() as i64, input_span),
                                    Value::int(cap.start() as i64 + 1, input_span),
                                    Value::int(cap.end() as i64, input_span),
                                    RangeInclusion::RightExclusive,
                                    input_span
                                ).unwrap().into(),
                                input_span
                            ),
                            "text" => Value::string(cap.as_str(), input_span),
                        ),
                        input_span,
                    ));
                }
            }
            result.push(Value::list(captures, input_span));
        }

        Ok(Value::list(result, input_span))
    }
}
