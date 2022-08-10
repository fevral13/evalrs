#[cfg(test)]
mod test {
    use js_sandbox::Script;
    use serde_json::Value;
    use std::collections::HashMap;

    const SCRIPT: &'static str =
        r#"function wrapper(script_snippet, {arg1, arg2}){ return eval(script_snippet) } "#;

    #[test]
    fn test_script_format() {
        let mut s = Script::from_string(&SCRIPT).unwrap();
        let args = ("arg1+arg2", HashMap::from([("arg1", 1), ("arg2", 5)]));
        let result: Value = s.call("wrapper", args).unwrap();
        assert_eq!(Value::from(6), result);
    }
}
