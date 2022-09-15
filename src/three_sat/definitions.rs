use std::collections::HashMap;
use std::fmt;

pub type Variable = String;

#[derive(Clone, Debug)]
pub enum Literal {
    Pos(Variable),
    Neg(Variable),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Pos(x) => write!(f, "{}",  x),
            Literal::Neg(x) => write!(f, "¬{}", x),
        }
    }
}

impl Literal {
    pub fn get_variable(&self) -> Variable {
        match self {
            Self::Pos(x) => x.to_string(),
            Self::Neg(x) => x.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Clause(pub Literal, pub Literal, pub Literal);

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Clause(l1, l2, l3) = self;

        write!(f, "({} ∨ {} ∨ {})", l1, l2, l3)
    }
}

#[derive(Debug)]
pub struct Formula {
    pub clauses: Vec<Clause>,
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iterator = self.clauses.iter().peekable();

        while let Some(clause) = iterator.next() {
            write!(f, "{}", clause)?;

            if !iterator.peek().is_none() {
                write!(f, " ∧ ")?;
            }
        }

        Ok(())
    }
}

// pub type Solution = HashMap<Variable, bool>;
#[derive(Debug)]
pub struct Solution(pub HashMap<Variable, bool>);

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let literals: Vec<(Variable, bool)> = self.0.iter().map(|(l, &v)| (l.clone(), v)).collect();

        let literals_strs = literals.iter()
                                    .map(|(literal, val)| if *val { format!("{}", literal) } else { format!("¬{}", literal) })
                                    .collect::<Vec<_>>();

        let mut result = String::new();

        for literal in &literals_strs[0..literals_strs.len() - 1] {
            result.push_str(&literal);
            result.push_str(", ");
        }

        result.push_str(&literals_strs[literals_strs.len() - 1].to_string());

        write!(f, "{{{}}}", result)
    }
}
