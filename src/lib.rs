use js_sandbox::Script;
use serde_json::Value;
use std::error::Error;

pub fn calc(script: &str, variables: &Value) -> Result<Value, Box<dyn Error>> {
    let code = format_script(script, &variables)?;

    let mut script = Script::from_string(&code)?;
    let result: Value = script.call("wrapper", &variables)?;
    Ok(result)
}

fn format_script(raw_code: &str, variables: &Value) -> Result<String, Box<dyn Error>> {
    let variable_names: String = match variables {
        Value::Object(obj) => Ok(obj.keys().cloned().collect::<Vec<String>>().join(", ")),
        _ => Err("Variables must be an object"),
    }?;

    Ok(format!(
        r#"wrapper = ({{ {variable_names} }}) => {{ {raw_code} }}"#,
        raw_code = raw_code,
        variable_names = variable_names
    ))
}

#[cfg(test)]
mod test {
    use crate::{calc, format_script};
    use serde_json::json;

    #[test]
    fn test_script_format() {
        let raw_code = "return a";
        let variables = json!({"a": 2, "c": 9});
        let rendered_script = format_script(&raw_code, &variables).unwrap();
        let expected = String::from("wrapper = ({ a, c }) => { return a }");
        assert_eq!(expected, rendered_script);
    }

    #[test]
    fn test_calc() {
        let raw_code = "return a";
        let variables = json!({"a": {"b": 3}});
        let result = calc(&raw_code, &variables);
        let expected = json!({"b": 3});

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_malformed_script() {
        let raw_code = "aaaaaa!";
        let variables = json!({"a": {"b": 3}});
        let result = calc(&raw_code, &variables);
        let expected = "Uncaught SyntaxError: Unexpected token '!'\n    at sandboxed.js:1:110";
        let err = result.unwrap_err().to_string();
        assert_eq!(expected, err);
    }
}
