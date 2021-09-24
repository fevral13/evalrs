use js_sandbox::AnyError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalrsError {
    #[error("Variables must be an object")]
    WrongVariablesType,

    #[error("The code is not valid JS")]
    WrongScriptCode { source: AnyError },

    #[error("Error evaluating script")]
    ScriptEvaluationError { source: AnyError },

    #[error("Script not cached and no Id submitted")]
    NoIdNorScriptSubmitted,

    #[error("Script Id not found")]
    IdNotFound,
}
