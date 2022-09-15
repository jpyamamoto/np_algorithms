use super::definitions::*;
use petgraph::matrix_graph::MatrixGraph as G;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

const MIN_VERTICES: usize = 10;
const MAX_VERTICES: usize = 20;

pub fn generate_input() -> Input {
    let mut rng = thread_rng();

    let num_vertices: usize = rng.gen_range(MIN_VERTICES..=MAX_VERTICES);
    let mut graph = G::new_undirected();

    let vertices = (b'a'..(((b'a' as usize) + num_vertices) as u8))
        .map(|name| graph.add_node((name as char).to_string()))
        .collect::<Vec<_>>();


    for i in 0..vertices.len() {
        let v1 = vertices[i];

        for j in (i+1)..vertices.len() {
            if rng.gen_bool(0.25) {
                let v2 = vertices[j];
                graph.add_edge(v1, v2, format!("{} - {}", graph[v1], graph[v2]));
            }
        }
    }

    let chosen = vertices.choose_multiple(&mut rng, 2)
                         .cloned().collect::<Vec<_>>();

    (chosen[0], chosen[1], Graph(graph))
}
