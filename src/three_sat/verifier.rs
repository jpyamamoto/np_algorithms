use super::definitions::*;

pub fn verify(formula: &Formula, solution: &Solution) -> bool {
    formula
        .clauses
        .iter()
        .all(|clause| verify_clause(clause, solution))
}

fn verify_clause(clause: &Clause, solution: &Solution) -> bool {
    let Clause(l1, l2, l3) = clause;

    vec![l1, l2, l3].iter().any(|&l| value_literal(l, &solution))
}

fn value_literal(literal: &Literal, solution: &Solution) -> bool {
    match literal {
        Literal::Pos(var) => *solution.0.get(var).unwrap(),
        Literal::Neg(var) => !solution.0.get(var).unwrap(),
    }
}
