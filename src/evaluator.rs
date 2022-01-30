use actix_web::web::Data;
use js_sandbox::Script;
use log::{debug, warn};
use serde_json::Value;

use crate::app_state::AppState;
use crate::cache_backend::CacheBackend;
use crate::errors::EvalrsError;
use crate::js_prelude::JS_PRELUDE;
use crate::request::Request;

pub struct EvaluationOk {
    pub result: Value,
}

pub fn evaluate(request: &Request, data: &mut Data<AppState>) -> Result<EvaluationOk, EvalrsError> {
    let cache = &mut *data.cache.lock().unwrap(); // fixme Learn how to use mutexes!
    let script_code = get_script_from_cache(&request.id, &request.script, cache)?;

    debug!(
        "Trying to evaluate script:\n{}\nwith variables:\n{}",
        &script_code,
        serde_json::to_string(&request.variables).unwrap_or_default()
    );

    let mut script_evaluator = get_script_evaluator(&request.variables)?;

    let timeout = match request.timeout {
        Some(timeout) => timeout,
        None => data.settings.js.default_timeout,
    };

    let args = Value::Array(vec![
        Value::String(script_code.clone()),
        request.variables.clone(),
    ]);

    match script_evaluator.call::<Value, Value>("wrapper", &args, Some(timeout)) {
        Ok(result) => {
            debug!(
                "Script evaluated successfully with result {}",
                serde_json::to_string(&result).unwrap_or_default()
            );
            Ok(EvaluationOk { result })
        }
        Err(error) => {
            warn!("Script evaluation failure: {}", &error.to_string());
            Err(EvalrsError::ScriptEvaluationError { source: error })
        }
    }
}

fn get_script_from_cache<'a>(
    id: &'a Option<String>,
    script: &'a Option<String>,
    cache: &'a mut dyn CacheBackend,
) -> Result<&'a String, EvalrsError> {
    match script {
        Some(script_code) => {
            debug!("Script submitted");
            if let Some(id_value) = id {
                debug!("Saving submitted script with id {}", id_value);
                cache.set(id_value, script_code);
            }

            return Ok(script_code);
        }
        None => {
            debug!("Script is not submitted");

            match id {
                Some(id_value) => {
                    debug!("Looking up in cache with id {}", id_value);
                    match cache.get(id_value) {
                        Some(script_code) => {
                            debug!("Script with id {} found", id_value);
                            Ok(script_code)
                        }
                        None => {
                            debug!("Script with id {} not found", id_value);
                            Err(EvalrsError::IdNotFound)
                        }
                    }
                }
                None => {
                    warn!("No script nor id submitted");
                    Err(EvalrsError::NoIdNorScriptSubmitted)
                }
            }
        }
    }
}

fn get_script_evaluator(variables: &Value) -> Result<Script, EvalrsError> {
    let arguments: String = match variables {
        Value::Object(object) => Ok(object.keys().cloned().collect::<Vec<String>>().join(", ")),
        _ => Err(EvalrsError::WrongVariablesType),
    }?;

    let raw_script = format!(
        r#" {prelude} function wrapper([script_snippet, {{ {arguments} }} ]){{ return eval(script_snippet) }} "#,
        prelude = JS_PRELUDE,
        arguments = arguments,
    );

    match Script::from_string(&raw_script) {
        Ok(script) => Ok(script),
        Err(err) => Err(EvalrsError::WrongScriptCode { source: err }),
    }
}
