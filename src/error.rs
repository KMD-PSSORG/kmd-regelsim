use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Cyclic dependency detected: {cycle}")]
    CyclicDependency { cycle: String },

    #[error("Invalid parameter: {name} = {value}")]
    InvalidParameter { name: String, value: f64 },

    #[error("Unknown rule: {rule_id}")]
    UnknownRule { rule_id: String },
}
