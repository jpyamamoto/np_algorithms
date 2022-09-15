use petgraph::Undirected;
use petgraph::matrix_graph::{MatrixGraph, NodeIndex};

pub type GraphMatrix = MatrixGraph<String, String, Undirected>;

pub struct Graph(pub GraphMatrix);

pub type Input = (NodeIndex, NodeIndex, Graph);

pub struct Solution(pub Vec<NodeIndex>);

impl Solution {
    pub fn show(&self, graph: &Graph) -> String {
        let Graph(graph) = graph;

        let vectors: Vec<String> = self.0.iter()
                                         .map(|&v| graph[v].to_string())
                                         .collect();

        vectors.join(" -> ")
    }
}
