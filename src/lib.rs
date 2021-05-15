use risp::types::RispType;
use serde_yaml::Value;

pub fn convert(yaml_term: &Value) -> RispType {
    match yaml_term {
        Value::Null => RispType::Nil,
        Value::String(var) => RispType::Symbol(var.to_string()),
        Value::Bool(var) => RispType::Bool(*var),
        Value::Number(var) => RispType::Int(var.as_i64().unwrap()),
        // TODO mappings could be used to apply functions
        Value::Mapping(_) => RispType::Str("TODO mappings".to_string()),
        Value::Sequence(vec) => match vec.as_slice() {
            [] => RispType::Str("TODO empty sequence".to_string()),
            [Value::String(ref def), Value::String(name), val] if def == "def" => {
                RispType::List(vec![
                    RispType::Symbol(def.to_string()),
                    RispType::Symbol(name.to_string()),
                    convert(val),
                ])
            }
            [Value::String(ref defn), Value::String(name), Value::Sequence(params), val @ Value::Sequence(_)]
                if defn == "defn" =>
            {
                RispType::List(vec![
                    RispType::Symbol(defn.to_string()),
                    RispType::Symbol(name.to_string()),
                    RispType::Vector(params.iter().map(|term| convert(term)).collect()),
                    convert(val),
                ])
            }
            // here we take on rust's syntax to distinguish lists and vectors
            [Value::String(ref term0), terms @ ..] if term0 == "vec!" => {
                RispType::Vector(terms.iter().map(|term| convert(term)).collect())
            }
            terms => RispType::List(terms.iter().map(|term| convert(term)).collect()),
        },
    }
}
