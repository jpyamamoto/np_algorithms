use super::definitions::*;
use std::collections::{HashMap, HashSet};

pub fn guess(input: &Formula) -> Solution {
    let solution_table = retrieve_variables(input)
        .iter()
        .fold(HashMap::new(), |mut solution, variable| {
            solution.insert(variable.to_string(), rand::random());
            solution
        });

    Solution(solution_table)
}

fn retrieve_variables(formula: &Formula) -> HashSet<Variable> {
    formula
        .clauses
        .iter()
        .fold(HashSet::new(), |mut variables, Clause(l1, l2, l3)| {
            variables.insert(l1.get_variable());
            variables.insert(l2.get_variable());
            variables.insert(l3.get_variable());
            variables
        })
}
