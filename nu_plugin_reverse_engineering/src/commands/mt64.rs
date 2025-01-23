use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{
    LabeledError, ListStream, PipelineData, Signals, Signature, Span, SyntaxShape, Type, Value,
};
use rand_mt::Mt64;

use crate::ReverseEngineeringPlugin;

pub struct Mt64Command;

impl PluginCommand for Mt64Command {
    type Plugin = ReverseEngineeringPlugin;
    fn name(&self) -> &str {
        "mt64"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_type(Type::Nothing, Type::list(Type::Int))
            .named(
                "seed",
                SyntaxShape::Int,
                "seed for the random number generator",
                Some('s'),
            )
            .named(
                "count",
                SyntaxShape::Int,
                "number of random numbers to generate",
                Some('c'),
            )
    }

    fn description(&self) -> &str {
        "Generate random numbers using the Mersenne Twister 64-bit algorithm"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let seed = call
            .get_flag_value("seed")
            .map(|s| s.as_int().unwrap().to_le_bytes());
        let count = call
            .get_flag_value("count")
            .map(|c| c.as_int().unwrap())
            .unwrap_or(1) as u64;

        let rng = seed.map(Mt64::from).unwrap_or(Mt64::new_unseeded());

        let iter = Mt64Iter {
            rng,
            count,
            span: call.head,
        };

        let list_stream = ListStream::new(iter, call.head, Signals::empty());

        Ok(PipelineData::ListStream(list_stream, None))
    }
}

struct Mt64Iter {
    rng: Mt64,
    count: u64,
    span: Span,
}

impl Iterator for Mt64Iter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }

        self.count -= 1;
        let n = self.rng.next_u64();
        Some(Value::int(i64::from_le_bytes(n.to_le_bytes()), self.span))
    }
}
