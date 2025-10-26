use crate::types::{DepEdge, ModuleIR};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

pub struct DependencyGraph {
    graph: DiGraph<String, DepEdge>,
    path_to_node: HashMap<String, NodeIndex>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            path_to_node: HashMap::new(),
        }
    }

    pub fn add_module(&mut self, module: &ModuleIR) -> NodeIndex {
        if let Some(&node) = self.path_to_node.get(&module.path) {
            return node;
        }

        let node = self.graph.add_node(module.path.clone());
        self.path_to_node.insert(module.path.clone(), node);
        node
    }

    pub fn add_edge(&mut self, source: &str, target: &str, edge: DepEdge) {
        let source_node = self.path_to_node.get(source);
        let target_node = self.path_to_node.get(target);

        if let (Some(&src), Some(&tgt)) = (source_node, target_node) {
            self.graph.add_edge(src, tgt, edge);
        }
    }

    pub fn get_outgoing(&self, path: &str) -> Vec<&DepEdge> {
        if let Some(&node) = self.path_to_node.get(path) {
            self.graph
                .edges(node)
                .map(|e| e.weight())
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_incoming(&self, path: &str) -> Vec<&DepEdge> {
        if let Some(&node) = self.path_to_node.get(path) {
            self.graph
                .edges_directed(node, petgraph::Direction::Incoming)
                .map(|e| e.weight())
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn fan_in(&self, path: &str) -> u32 {
        if let Some(&node) = self.path_to_node.get(path) {
            self.graph
                .edges_directed(node, petgraph::Direction::Incoming)
                .count() as u32
        } else {
            0
        }
    }

    pub fn fan_out(&self, path: &str) -> u32 {
        if let Some(&node) = self.path_to_node.get(path) {
            self.graph.edges(node).count() as u32
        } else {
            0
        }
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "graph_test.rs"]
mod graph_test;
