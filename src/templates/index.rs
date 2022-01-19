pub const INDEX_TEMPLATE: &str = r#"
    <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>eval-rs</title>
</head>
<body>
<h1>Usage:</h1>
<h2>Request</h2>
<p>POST /eval/</p>
<pre>JSON:
{
    "script": "&lt;js script code&gt;",
    "variables": {&lt;object of variables&gt;},
    "timeout": &lt;int in milliseconds&gt;
}
</pre>
<h2>Response</h2>
<h3>Success:</h3>
<pre>
Status code: 200
{
    "result": &lt;execution result (a scalar value, a list, an object or null if no value returned)&gt;
}
</pre>
<h3>Failure</h3>
<pre>
Status code: 400
{
    "message": "&lt;error description&gt;"
}
</pre>
</body>
</html>"#;
