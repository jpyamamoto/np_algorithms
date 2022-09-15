use super::definitions::*;
use rand::seq::SliceRandom;
use rand::{thread_rng, random};

const NUM_VARS: usize = 10;
const NUM_CLAUSES: usize = 5;
const CLAUSE_SIZE: usize = 3;

pub fn generate_input() -> Formula {
    let mut variables: Vec<String> = (b'a'..(((b'a' as usize) + NUM_VARS) as u8))
        .map(|c| (c as char).to_string())
        .collect();

    let mut rng = thread_rng();
    variables.shuffle(&mut rng);

    variables = variables.iter().cycle().take(NUM_CLAUSES * CLAUSE_SIZE).cloned().collect();
    variables.shuffle(&mut rng);

    let literals = variables.iter()
                            .map(|l| if random() { Literal::Pos(l.to_string()) } else { Literal::Neg(l.to_string()) })
                            .collect::<Vec<_>>();

    let clauses: Vec<Clause> = literals.chunks(CLAUSE_SIZE)
                                       .map(|ls| Clause(ls[0].clone(), ls[1].clone(), ls[2].clone()))
                                       .collect();

    Formula { clauses }
}
