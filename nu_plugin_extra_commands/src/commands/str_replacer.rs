use std::vec;

use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{
    ast::RangeInclusion, record, IntRange, LabeledError, Signature, SyntaxShape, Type, Value,
};
use regex::{Captures, Regex};

use crate::ExtraCommandsPlugin;

pub struct StrReplacer;

impl SimplePluginCommand for StrReplacer {
    type Plugin = ExtraCommandsPlugin;

    fn name(&self) -> &str {
        "str replacer"
    }

    fn description(&self) -> &str {
        "Replaces all matches of a regex pattern with the result of a closure"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::String, Type::String)
            .required("regex", SyntaxShape::String, "the regex pattern to match")
            .required(
                "replacer",
                SyntaxShape::Closure(Some(vec![match_result_type().to_shape()])),
                "the closure to use for replacement",
            )
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
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

        let replacer = call.req(1)?;

        let result = replace_all(&re, input, |caps: &regex::Captures| {
            let caps = caps
                .iter()
                .filter_map(|capture| {
                    if let Some(cap) = capture {
                        Some(Value::record(
                            record!(
                                "range" => Value::range(
                                    IntRange::new(
                                        Value::int(cap.start() as i64, input_span),
                                        Value::int(cap.start() as i64 + 1, input_span),
                                        Value::int(cap.end() as i64, input_span),
                                        RangeInclusion::RightExclusive,
                                        input_span,
                                    ).unwrap().into(),
                                    input_span,
                                ),
                                "text" => Value::string(cap.as_str(), input_span),
                            ),
                            input_span,
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let match_result = Value::list(caps, input_span);

            let result =
                engine.eval_closure(&replacer, vec![match_result.clone()], Some(match_result));

            match result {
                Ok(Value::String { val, .. }) => Ok(val),
                Ok(value) => Err(LabeledError::new("Expected String output from closure")
                    .with_label(
                        format!("requires string output; got {}", value.get_type()),
                        replacer.span,
                    )),
                Err(e) => Err(LabeledError::new("Error evaluating closure")
                    .with_label(format!("Error: {}", e), replacer.span)),
            }
        });

        match result {
            Ok(result) => Ok(Value::string(result, input_span)),
            Err(e) => Err(e),
        }
    }
}

// https://docs.rs/regex/latest/regex/struct.Regex.html#fallibility
fn replace_all<E>(
    re: &Regex,
    haystack: &str,
    replacement: impl Fn(&Captures) -> Result<String, E>,
) -> Result<String, E> {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;
    for caps in re.captures_iter(haystack) {
        let m = caps.get(0).unwrap();
        new.push_str(&haystack[last_match..m.start()]);
        new.push_str(&replacement(&caps)?);
        last_match = m.end();
    }
    new.push_str(&haystack[last_match..]);
    Ok(new)
}

pub fn match_result_type() -> Type {
    Type::list(Type::Record(Box::new([
        ("range".into(), Type::Range),
        ("text".into(), Type::String),
    ])))
}
