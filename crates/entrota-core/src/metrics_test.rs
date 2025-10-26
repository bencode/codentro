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
    fn test_calculate_entropy_empty_module() {
        let module = ModuleIR {
            path: "test.ts".to_string(),
            language: Some("typescript".to_string()),
            loc: 0,
            entropy: 0.0,
            symbols: vec![],
            outgoing: vec![],
            incoming: vec![],
        };

        let entropy = calculate_entropy(&module);
        assert_eq!(entropy, 0.0);
    }

    #[test]
    fn test_calculate_entropy_with_symbols() {
        let module = ModuleIR {
            path: "test.ts".to_string(),
            language: Some("typescript".to_string()),
            loc: 100,
            entropy: 0.0,
            symbols: vec![
                Symbol {
                    kind: SymbolKind::Function,
                    name: "test".to_string(),
                    loc: 50,
                    entropy: None,
                },
            ],
            outgoing: vec![],
            incoming: vec![],
        };

        let entropy = calculate_entropy(&module);
        assert!(entropy > 0.0);
        assert!(entropy <= 1.0);
    }

    #[test]
    fn test_calculate_symbol_entropy() {
        let small_symbol = Symbol {
            kind: SymbolKind::Function,
            name: "small".to_string(),
            loc: 5,
            entropy: None,
        };

        let large_symbol = Symbol {
            kind: SymbolKind::Function,
            name: "large".to_string(),
            loc: 100,
            entropy: None,
        };

        let small_entropy = calculate_symbol_entropy(&small_symbol);
        let large_entropy = calculate_symbol_entropy(&large_symbol);

        assert!(small_entropy < large_entropy);
        assert!(large_entropy <= 1.0);
    }

    #[test]
    fn test_generate_suggestions_high_entropy() {
        let module = ModuleIR {
            path: "test.ts".to_string(),
            language: Some("typescript".to_string()),
            loc: 100,
            entropy: 0.8,
            symbols: vec![],
            outgoing: vec![],
            incoming: vec![],
        };

        let suggestions = generate_suggestions(&module, 2, 3);
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("High entropy")));
    }

    #[test]
    fn test_generate_suggestions_high_fanout() {
        let module = ModuleIR {
            path: "test.ts".to_string(),
            language: Some("typescript".to_string()),
            loc: 100,
            entropy: 0.5,
            symbols: vec![],
            outgoing: vec![],
            incoming: vec![],
        };

        let suggestions = generate_suggestions(&module, 2, 15);
        assert!(suggestions.iter().any(|s| s.contains("fan-out")));
    }
}
