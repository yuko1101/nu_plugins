use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{
    ast::RangeInclusion, record, IntRange, LabeledError, Signature, Span, SyntaxShape, Type, Value,
};
use regex::{Captures, Match, Regex};

use crate::ExtrasPlugin;
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

        let results: Vec<Value> = re
            .captures_iter(input)
            .map(|caps| get_match_result(&re, &caps, input_span))
            .collect();

        Ok(Value::list(results, input_span))
    }
}

pub fn get_match_result(re: &Regex, caps: &Captures, span: Span) -> Value {
    let capture_names = re
        .capture_names()
        .enumerate()
        .map(|(i, name)| name.map(String::from).unwrap_or_else(|| i.to_string()))
        .collect::<Vec<_>>();

    let mut captures = vec![];

    for (cap_index, cap) in caps.iter().enumerate() {
        if let Some(cap) = cap {
            captures.push(get_match_result_cap(&capture_names[cap_index], cap, span));
        }
    }

    Value::list(captures, span)
}

fn get_match_result_cap(name: impl Into<String>, cap: Match, span: Span) -> Value {
    Value::record(
        record!(
            "name" => Value::string(name, span),
            "text" => Value::string(cap.as_str(), span),
            "range" => Value::range(
                IntRange::new(
                    Value::int(cap.start() as i64, span),
                    Value::int(cap.start() as i64 + 1, span),
                    Value::int(cap.end() as i64, span),
                    RangeInclusion::RightExclusive,
                    span
                ).unwrap().into(),
                span
            ),
        ),
        span,
    )
}

pub fn match_result_type() -> Type {
    Type::list(Type::Record(Box::new([
        ("name".into(), Type::String),
        ("text".into(), Type::String),
        ("range".into(), Type::Range),
    ])))
}
