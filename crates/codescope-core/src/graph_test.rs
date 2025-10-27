#[cfg(test)]
mod tests {
    use crate::graph::DependencyGraph;
    use crate::types::{DepEdge, DepKind, ModuleIR};

    fn create_test_module(path: &str) -> ModuleIR {
        ModuleIR {
            path: path.to_string(),
            language: Some("typescript".to_string()),
            loc: 100,
            comment_lines: 10,
            blank_lines: 5,
            symbols: vec![],
            metrics: vec![],
            outgoing: vec![],
            incoming: vec![],
        }
    }

    #[test]
    fn test_add_module() {
        let mut graph = DependencyGraph::new();
        let module = create_test_module("test.ts");

        graph.add_module(&module);
        assert_eq!(graph.fan_in("test.ts"), 0);
        assert_eq!(graph.fan_out("test.ts"), 0);
    }

    #[test]
    fn test_add_edge() {
        let mut graph = DependencyGraph::new();

        let mod_a = create_test_module("a.ts");
        let mod_b = create_test_module("b.ts");

        graph.add_module(&mod_a);
        graph.add_module(&mod_b);

        let edge = DepEdge {
            source: Some("a.ts".to_string()),
            target: Some("b.ts".to_string()),
            relation: DepKind::Import,
            strength: 0.7,
            files: None,
        };

        graph.add_edge("a.ts", "b.ts", edge);

        assert_eq!(graph.fan_out("a.ts"), 1);
        assert_eq!(graph.fan_in("b.ts"), 1);
    }

    #[test]
    fn test_get_outgoing() {
        let mut graph = DependencyGraph::new();

        let mod_a = create_test_module("a.ts");
        let mod_b = create_test_module("b.ts");

        graph.add_module(&mod_a);
        graph.add_module(&mod_b);

        let edge = DepEdge {
            source: Some("a.ts".to_string()),
            target: Some("b.ts".to_string()),
            relation: DepKind::Import,
            strength: 0.7,
            files: None,
        };

        graph.add_edge("a.ts", "b.ts", edge);

        let outgoing = graph.get_outgoing("a.ts");
        assert_eq!(outgoing.len(), 1);
        assert_eq!(outgoing[0].strength, 0.7);
    }

    #[test]
    fn test_get_incoming() {
        let mut graph = DependencyGraph::new();

        let mod_a = create_test_module("a.ts");
        let mod_b = create_test_module("b.ts");

        graph.add_module(&mod_a);
        graph.add_module(&mod_b);

        let edge = DepEdge {
            source: Some("a.ts".to_string()),
            target: Some("b.ts".to_string()),
            relation: DepKind::Import,
            strength: 0.7,
            files: None,
        };

        graph.add_edge("a.ts", "b.ts", edge);

        let incoming = graph.get_incoming("b.ts");
        assert_eq!(incoming.len(), 1);
    }

    #[test]
    fn test_multiple_edges() {
        let mut graph = DependencyGraph::new();

        graph.add_module(&create_test_module("a.ts"));
        graph.add_module(&create_test_module("b.ts"));
        graph.add_module(&create_test_module("c.ts"));

        graph.add_edge("a.ts", "b.ts", DepEdge {
            source: Some("a.ts".to_string()),
            target: Some("b.ts".to_string()),
            relation: DepKind::Import,
            strength: 0.7,
            files: None,
        });

        graph.add_edge("a.ts", "c.ts", DepEdge {
            source: Some("a.ts".to_string()),
            target: Some("c.ts".to_string()),
            relation: DepKind::Import,
            strength: 0.8,
            files: None,
        });

        assert_eq!(graph.fan_out("a.ts"), 2);
        assert_eq!(graph.fan_in("b.ts"), 1);
        assert_eq!(graph.fan_in("c.ts"), 1);
    }
}
