use std::collections::HashSet;

use crate::{input_parsing::erasable_cluster::ErasableCluster, shared::sign::Sign};

use super::{
    calculation_precision::{FloatingPointPrecison, UnsignedValuePrecision},
    parsers::parse_into_expression,
};

pub(super) type Expression = Vec<Term>;

fn expression_to_inexact(expression: &Expression) -> FloatingPointPrecison {
    // expression
    //     .iter()
    //     .map(|term| simplify_term(term).into())
    //     .reduce();
    todo!()
}

#[derive(Debug)]
pub(crate) struct Term {
    pub(super) fragments: Vec<TermFragment>,
}
impl Into<FloatingPointPrecison> for &Term {
    fn into(self) -> FloatingPointPrecison {
        self.fragments
            .iter()
            .map(|fragment| <&TermFragment as Into<FloatingPointPrecison>>::into(fragment))
            .reduce(|a, b| a * b)
            .unwrap()
    }
}

#[derive(Debug)]
pub(super) struct TermFragment {
    pub(super) sign: Sign,
    pub(super) fragment_magnitude: TermFragmentMagnitude,
    pub(super) multiplied_or_divided: MultipliedOrDivided,
    pub(super) angle_unit: AngleUnit,
}
impl Into<FloatingPointPrecison> for &TermFragment {
    fn into(self) -> FloatingPointPrecison {
        // match self.fragment_magnitude {}
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum AngleUnit {
    Degrees,
    Radians,
    Non,
}

#[derive(Debug)]
pub(super) enum TermFragmentMagnitude {
    NonNamedConstant(NonNamedConstant),
    Bracket(Expression),
    NamedConstant {
        coefficient: Expression,
        constant: NamedConstant,
    },
    Function(Function),
    Inexact(FloatingPointPrecison),
}

#[derive(Debug)]
pub(super) enum NamedConstant {
    Pi,
    E,
    I,
}

enum Angle {
    Degrees(Expression),
    Radians(Expression),
}

#[derive(Debug)]
pub(super) enum NonNamedConstant {
    Integer(UnsignedValuePrecision),
    Fraction {
        numerator: Expression,
        denominator: Expression,
    },
    Decimal {
        before_decimal_point: String,
        after_decimal_point: String,
    },
    Power {
        base: Expression,
        exponent: Expression,
    },
}

// used for calculations
#[derive(Debug)]
pub(super) enum Function {
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
impl Into<FloatingPointPrecison> for Function {
    fn into(self) -> FloatingPointPrecison {
        // match self {
        //     Self::NthRoot(n, value_under_root) => {}
        // }
        todo!()
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub(super) enum MultipliedOrDivided {
    #[default]
    Multiplied,
    Divided,

    // used to signify the beginning of a term
    Neither,
}

#[derive(Debug)]
enum OutputModes {
    ExactImproperFractionRadians,
    ExactImproperFractionDegrees,
    ExactMixedFractionRadians,
    ExactMixedFractionDegrees,
    DecimalRadians,
    DecimalDegrees,
}

#[derive(Debug)]
pub struct Calculator {
    expression: Expression,
    output_modes: HashSet<OutputModes>,
}

fn simplify_term_fragment(fragment: &TermFragment) -> TermFragment {
    TermFragment {
        fragment_magnitude: match &fragment.fragment_magnitude {
            TermFragmentMagnitude::Bracket(expression) => {
                TermFragmentMagnitude::Bracket(simplify_expression(expression))
            }
            // TermFragmentMagnitude::Function(function) => {}
            _ => todo!(),
        },
        ..(*fragment)
    }
}

fn simplify_term(term: &Term) -> Term {
    let simplified_fragments: Vec<TermFragment> =
        term.fragments.iter().map(simplify_term_fragment).collect();

    todo!()
}

fn simplify_expression(expression: &Expression) -> Expression {
    todo!()
}

impl From<&ErasableCluster> for Calculator {
    fn from(cluster: &ErasableCluster) -> Self {
        let iterator = cluster.iter();

        Calculator {
            expression: parse_into_expression(iterator),
            output_modes: HashSet::new(),
        }
    }
}
