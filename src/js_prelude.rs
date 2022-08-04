pub const JS_PRELUDE: &str = r#"
function sqrt(x) {
    if (!isNumeric(x) || x < 0) {
        throw new InvalidFunctionCall(sqrt, x);
    }
    return Math.sqrt(x);
}

function pow(x, n) {
    if (!isNumeric(x) || !isNumeric(n) || (x < 0 && Math.abs(n) < 1) || (x === 0 && n < 0)) {
        throw new InvalidFunctionCall(pow, x, n);
    }
    return Math.pow(x, n);
}

function min(...args) {
    const filteredArgs = args.filter(x => x !== null);
    if (filteredArgs.some((x) => !isNumeric(x)) || filteredArgs.length === 0) {
        throw new InvalidFunctionCall(min, ...args);
    }
    return Math.min(...filteredArgs);
}

function max(...args) {
    const filteredArgs = args.filter(x => x !== null);
    if (filteredArgs.some((x) => !isNumeric(x)) || filteredArgs.length === 0) {
        throw new InvalidFunctionCall(max, ...args);
    }
    return Math.max(...filteredArgs);
}

function round(n, k) {
    if (!isNumeric(n) || !isNumeric(k)) {
        throw new InvalidFunctionCall(round, n, k);
    }
    return Number(n.toFixed(k));
}

function if_g_le(val, lower, upper, res) {
    if (!isNumeric(val) || !isNumeric(lower) || !isNumeric(upper) || lower >= upper) {
        throw new InvalidFunctionCall(if_g_le, val, lower, upper, res);
    }
    return (lower < val && val <= upper) ? res : null;
}

function isNumeric(n) {
    return n === Number.parseFloat(n) && !Number.isNaN(n) && Number.isFinite(n);
}

class InvalidFunctionCall extends Error {
    constructor(func, ...args) {
        const formattedArgs = args.map(x => typeof x === 'string' ? `'${"$"}{x}'` : x);
        const message = `Invalid function call: ${"$"}{func.name}(${"$"}{formattedArgs.join(', ')})`;
        super(message);
    }
}
"#;
