use actix_web::web::Data;
use js_sandbox::Script;
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
    let script_code = get_script(&request.key, &request.script, cache)?;

    let mut script = get_compiled_script(script_code, &request.variables)?;

    let timeout = match request.timeout {
        Some(timeout) => timeout,
        None => data.settings.js.default_timeout,
    };

    match script.call::<Value, Value>("wrapper", &request.variables, Some(timeout)) {
        Ok(result) => Ok(EvaluationOk { result }),
        Err(error) => Err(EvalrsError::ScriptEvaluationError { source: error }),
    }
}

fn get_script<'a>(
    key: &'a Option<String>,
    script: &'a Option<String>,
    cache: &'a mut dyn CacheBackend,
) -> Result<&'a String, EvalrsError> {
    match script {
        Some(script_code) => {
            if let Some(key_value) = key {
                cache.set(key_value, script_code);
            }

            return Ok(script_code);
        }
        None => (),
    };

    match key {
        Some(key_value) => match cache.get(key_value) {
            Some(script_code) => Ok(script_code),
            None => Err(EvalrsError::KeyNotFound),
        },
        None => Err(EvalrsError::NoKeyNorScriptSubmitted),
    }
}

fn get_compiled_script(script_snippet: &str, variables: &Value) -> Result<Script, EvalrsError> {
    let arguments: String = match variables {
        Value::Object(object) => Ok(object.keys().cloned().collect::<Vec<String>>().join(", ")),
        _ => Err(EvalrsError::WrongVariablesType),
    }?;

    let raw_script = format!(
        r#" {prelude} function wrapper( {{ {arguments} }} ){{ return eval('{script_snippet}') }} "#,
        prelude = JS_PRELUDE,
        script_snippet = script_snippet,
        arguments = arguments,
    );

    match Script::from_string(&raw_script) {
        Ok(script) => Ok(script),
        Err(err) => Err(EvalrsError::WrongScriptCode { source: err }),
    }
}
