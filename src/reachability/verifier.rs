use super::definitions::*;

pub fn verify(input: &Input, solution: &Solution) -> bool {
    let (source, target, Graph(graph)) = input;

    if solution.0.len() < 2 {
        return false;
    }

    if *source != solution.0[0] {
        return false;
    }

    if *target != solution.0[solution.0.len() - 1] {
        return false;
    }

    solution.0.windows(2).all(|vecs| graph.has_edge(vecs[0], vecs[1]))
}
