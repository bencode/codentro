#[cfg(test)]
mod tests {
    use crate::metrics::*;
    use crate::types::{ModuleIR, Symbol, SymbolKind};

    #[test]
    fn test_count_lines_basic() {
        let source = r#"
// This is a comment
fn hello() {
    println!("world");
}
"#;
        let stats = count_lines(source);
        assert_eq!(stats.code, 3);
        assert_eq!(stats.comment, 1);
        assert_eq!(stats.blank, 1);
    }

    #[test]
    fn test_count_lines_block_comment() {
        let source = r#"
/*
 * Multi-line
 * comment
 */
fn test() {}
"#;
        let stats = count_lines(source);
        assert_eq!(stats.comment, 4);
        assert!(stats.code > 0);
    }

    #[test]
    fn test_generate_suggestions_large_file() {
        let module = ModuleIR {
            path: "test.ts".to_string(),
            language: Some("typescript".to_string()),
            loc: 600,
            comment_lines: 50,
            blank_lines: 30,
            symbols: vec![],
            metrics: vec![],
            outgoing: vec![],
            incoming: vec![],
        };

        let suggestions = generate_suggestions(&module, 2, 3);
        assert!(suggestions.iter().any(|s| s.contains("Large file")));
    }

    #[test]
    fn test_generate_suggestions_high_fanout() {
        let module = ModuleIR {
            path: "test.ts".to_string(),
            language: Some("typescript".to_string()),
            loc: 100,
            comment_lines: 10,
            blank_lines: 5,
            symbols: vec![],
            metrics: vec![],
            outgoing: vec![],
            incoming: vec![],
        };

        let suggestions = generate_suggestions(&module, 2, 15);
        assert!(suggestions.iter().any(|s| s.contains("fan-out")));
    }

    #[test]
    fn test_generate_suggestions_large_symbol() {
        let module = ModuleIR {
            path: "test.ts".to_string(),
            language: Some("typescript".to_string()),
            loc: 200,
            comment_lines: 10,
            blank_lines: 5,
            symbols: vec![Symbol {
                kind: SymbolKind::Function,
                name: "largeFunction".to_string(),
                loc: 100,
                cyclomatic_complexity: None,
                metrics: vec![],
            }],
            metrics: vec![],
            outgoing: vec![],
            incoming: vec![],
        };

        let suggestions = generate_suggestions(&module, 2, 3);
        assert!(suggestions
            .iter()
            .any(|s| s.contains("largeFunction") && s.contains("LOC")));
    }
}
