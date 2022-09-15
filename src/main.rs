mod three_sat;
mod reachability;

use petgraph::dot::Dot;

fn main() {
    println!("--- 3-SAT ---\n");
    sat_instance();
    println!("");

    println!("--- REACHABILITY ---\n");
    reachability_instance();
    println!("");
}

fn sat_instance() {
    let formula = three_sat::input::generate_input();

    let solution = three_sat::guesser::guess(&formula);
    let verification = three_sat::verifier::verify(&formula, &solution);

    println!("Entrada: {}", formula);
    println!("Solución Propuesta: {}", solution);
    println!("Verificación: {}", if verification { "Correcta" } else { "Incorrecta" });
}

fn reachability_instance() {
    let input = reachability::input::generate_input();

    let solution = reachability::guesser::guess(&input);
    let verification = reachability::verifier::verify(&input, &solution);
    let (source, target, graph) = input;

    println!("Entrada: (Origen = {}, Destino = {}, Gráfica = {})",
             graph.0[source], graph.0[target], Dot::new(&graph.0));
    println!("Solución Propuesta: {}", solution.show(&graph));
    println!("Verificación: {}", if verification { "Correcta" } else { "Incorrecta" });
}
