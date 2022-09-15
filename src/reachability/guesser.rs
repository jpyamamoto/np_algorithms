use super::definitions::*;
use petgraph::matrix_graph::NodeIndex;
use rand::seq::IteratorRandom;

pub fn guess(input: &Input) -> Solution {
    let (source, target, Graph(graph)) = input;
    let mut rng = rand::thread_rng();

    let mut graph = graph.clone();
    let mut node = *source;
    let target = *target;

    let mut path: Vec<NodeIndex> = vec![node];

    while node != target {
        let neighbours = graph.neighbors(node);

        if let Some(neighbour) = neighbours.choose(&mut rng) {
            path.push(neighbour);
            graph.remove_node(node);
            node = neighbour;
        } else {
            break;
        }
    }

    Solution(path)
}
