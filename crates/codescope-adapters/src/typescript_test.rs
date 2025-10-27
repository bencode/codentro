#[cfg(test)]
mod tests {
    use crate::typescript::TypeScriptAdapter;
    use crate::LanguageAdapter;
    use codescope_core::types::SymbolKind;

    #[test]
    fn test_complexity_simple_function() {
        let adapter = TypeScriptAdapter::new_typescript().unwrap();
        let source = r#"
function simple() {
    return 42;
}
"#;
        let result = adapter.parse(std::path::Path::new("test.ts"), source);
        assert!(result.is_ok());

        let module = result.unwrap();
        assert_eq!(module.symbols.len(), 1);
        assert_eq!(module.symbols[0].name, "simple");
        assert_eq!(module.symbols[0].cyclomatic_complexity, Some(1));
    }

    #[test]
    fn test_complexity_with_if_statements() {
        let adapter = TypeScriptAdapter::new_typescript().unwrap();
        let source = r#"
function withIf(x: number) {
    if (x > 0) {
        return x;
    } else {
        return -x;
    }
}
"#;
        let result = adapter.parse(std::path::Path::new("test.ts"), source);
        let module = result.unwrap();

        assert_eq!(module.symbols[0].cyclomatic_complexity, Some(2)); // 1 base + 1 if
    }

    #[test]
    fn test_complexity_with_loops() {
        let adapter = TypeScriptAdapter::new_typescript().unwrap();
        let source = r#"
function withLoop(arr: number[]) {
    let sum = 0;
    for (let i = 0; i < arr.length; i++) {
        sum += arr[i];
    }
    return sum;
}
"#;
        let result = adapter.parse(std::path::Path::new("test.ts"), source);
        let module = result.unwrap();

        assert_eq!(module.symbols[0].cyclomatic_complexity, Some(2)); // 1 base + 1 for
    }

    #[test]
    fn test_complexity_with_switch() {
        let adapter = TypeScriptAdapter::new_typescript().unwrap();
        let source = r#"
function withSwitch(x: number) {
    switch (x) {
        case 1:
            return "one";
        case 2:
            return "two";
        default:
            return "other";
    }
}
"#;
        let result = adapter.parse(std::path::Path::new("test.ts"), source);
        let module = result.unwrap();

        // 1 base + 2 cases (default doesn't count in standard cyclomatic complexity)
        assert_eq!(module.symbols[0].cyclomatic_complexity, Some(3));
    }

    #[test]
    fn test_complexity_with_logical_operators() {
        let adapter = TypeScriptAdapter::new_typescript().unwrap();
        let source = r#"
function withLogical(x: number, y: number) {
    if (x > 0 && y > 0) {
        return x + y;
    }
    return 0;
}
"#;
        let result = adapter.parse(std::path::Path::new("test.ts"), source);
        let module = result.unwrap();

        // 1 base + 1 if + 1 &&
        assert_eq!(module.symbols[0].cyclomatic_complexity, Some(3));
    }

    #[test]
    fn test_complexity_complex_function() {
        let adapter = TypeScriptAdapter::new_typescript().unwrap();
        let source = r#"
function complex(x: number, y: number, z: number) {
    if (x > 0) {
        if (y > 0) {
            if (z > 0) {
                return x + y + z;
            }
        }
    } else if (x < 0) {
        return -x;
    }

    for (let i = 0; i < 10; i++) {
        if (i % 2 === 0) {
            continue;
        }
    }

    return 0;
}
"#;
        let result = adapter.parse(std::path::Path::new("test.ts"), source);
        let module = result.unwrap();

        // 1 base + 3 nested ifs + 1 else if + 1 for + 1 inner if = 7
        assert_eq!(module.symbols[0].cyclomatic_complexity, Some(7));
    }

    #[test]
    fn test_non_function_symbols_have_no_complexity() {
        let adapter = TypeScriptAdapter::new_typescript().unwrap();
        let source = r#"
class MyClass {
    constructor() {}
}

interface MyInterface {
    x: number;
}

type MyType = string | number;
"#;
        let result = adapter.parse(std::path::Path::new("test.ts"), source);
        let module = result.unwrap();

        for symbol in &module.symbols {
            if symbol.kind != SymbolKind::Function {
                assert_eq!(symbol.cyclomatic_complexity, None);
            }
        }
    }
}
