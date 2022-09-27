use crate::{
    input_parsing::{
        erasable::{Erasable, ErasableType},
        erasable_cluster::ErasableCluster,
    },
    shared::{calculation_precision::UnsignedValuePrecision, sign::Sign},
};

use std::collections::HashSet;

type Expression = Vec<Term>;

struct Term {
    fragments: Vec<TermFragment>,
}

struct TermFragment {
    sign: Sign,
    fragment_magnitude: TermFragmentMagnitude,
    multiplied_or_divided: MultipliedOrDivided,
}

enum TermFragmentMagnitude {
    NonNamedConstant(NonNamedConstant),
    Bracket(Expression),
    NamedConstant {
        coefficient: Option<Expression>,
        constant: NamedConstant,
    },
    Function(Function),
}

enum NamedConstant {
    Pi,
    E,
    I,
}

enum AngleUnits {
    Degrees,
    Radians,
}

enum NonNamedConstant {
    Integer(UnsignedValuePrecision),
    Fraction {
        numerator: Expression,
        denominator: Expression,
    },
    Decimal {
        before_decimal_point: UnsignedValuePrecision,
        after_decimal_point: UnsignedValuePrecision,
    },
    NthRoot {
        degree: Expression,
        under_the_root: Expression,
    },
    Power {
        base: Expression,
        exponent: Expression,
    },
}

// used for calculations
enum Function {
    Absolute(Expression),
    Sin(Expression),
    Cos(Expression),
    Tan(Expression),
    Arcsin(Expression),
    Arccos(Expression),
    Arctan(Expression),
    // in the form NthRoot(n, value under the root)
    NthRoot(Expression, Expression),
}

enum MultipliedOrDivided {
    Multiplied,
    Divided,
}

// TODO: ParsingCalculator

enum OutputModes {
    ExactImproperFraction,
    ExactMixedFraction,
    Decimal,
}

pub struct ParsingCalculator {
    expression: Expression,
    output_modes: HashSet<OutputModes>,
}

impl From<&ErasableCluster> for ParsingCalculator {
    fn from(cluster: &ErasableCluster) -> Self {
        fn parse_term_fragmemt(
            iterator: &mut std::slice::Iter<'_, Erasable>,
        ) -> Option<TermFragment> {
            // while
            let mut iterator = iterator.peekable();

            if let Some(erasable) = iterator.peek() {
                let erasable = *erasable;
                let erasable_type: ErasableType = erasable.into();

                // match erasable_type {
                //     ErasableType::Digit => {
                //         let fragment_magnitude = TermFragmentMagnitude::NonNamedConstant(
                //             Non
                //         );

                //         while let Some(erasable) = iterator.peek() {
                //             let erasable = *erasable;
                //         }
                //     }
                // }
                todo!()
                //
            } else {
                None
            }
        }

        fn parse_term(iterator: &mut std::slice::Iter<'_, Erasable>) -> Option<Term> {
            let mut term = Term { fragments: vec![] };

            while let Some(term_fragment) = parse_term_fragmemt(iterator) {
                term.fragments.push(term_fragment);
            }

            if term.fragments.len() == 0 {
                None
            } else {
                Some(term)
            }
        }

        let mut iterator = cluster.iter();
        let mut expression = vec![];

        while let Some(term) = parse_term(&mut iterator) {
            expression.push(term);
        }

        ParsingCalculator {
            expression: vec![],
            output_modes: HashSet::new(),
        }
    }
}
