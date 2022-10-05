use std::{
    collections::HashSet,
    f64::consts::{E, PI},
    fmt::Debug,
    ops::Mul,
};

use crate::{
    input_parsing::erasable_cluster::ErasableCluster,
    shared::{errors::ParsingError, sign::Sign},
};

use super::{
    calculation_precision::{FloatingPointPrecison, UnsignedValuePrecision},
    inexact::{expression_to_inexact, Inexact},
    parsers::parse_into_expression,
    CalculationResult,
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub(super) type Expression = Vec<Term>;

#[derive(Debug)]
pub(crate) struct Term {
    pub(super) fragments: Vec<TermFragment>,
}

#[derive(Debug)]
pub(super) struct TermFragment {
    pub(super) sign: Sign,
    pub(super) fragment_magnitude: TermFragmentMagnitude,
    pub(super) multiplied_or_divided: MultipliedOrDivided,
    pub(super) angle_unit: Option<AngleUnit>,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub(crate) enum AngleUnit {
    Degrees,
    #[default]
    Radians,
}

#[derive(Debug)]
pub(super) enum TermFragmentMagnitude {
    NonNamedConstant(UnnamedConstant),
    Bracket(Expression),
    NamedConstant {
        coefficient: Expression,
        constant: NamedConstant,
    },
    Function(Function),
    // Inexact(FloatingPointPrecison),
}

#[derive(Debug)]
pub(super) enum NamedConstant {
    Pi,
    E,
}

enum Angle {
    Degrees(Expression),
    Radians(Expression),
}

#[derive(Debug)]
pub(super) enum UnnamedConstant {
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

#[derive(Default, Clone, Copy, Debug)]
pub(super) enum MultipliedOrDivided {
    #[default]
    Multiplied,
    Divided,

    // used to signify the beginning of a term
    Neither,
}

#[derive(EnumIter)]
enum InexactOutputMode {
    InexactRadians,
    InexactDegrees,
}

#[derive(EnumIter)]
enum ExactOutputMode {
    ExactImproperFractionRadians,
    ExactImproperFractionDegrees,
    ExactMixedFractionRadians,
    ExactMixedFractionDegrees,
}

#[derive(Debug)]
pub struct Calculator {
    expression: Expression,
    inexact_output_modes: InexactOutputModeIter,
    exact_output_modes: ExactOutputModeIter,
}

impl Debug for InexactOutputModeIter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "".fmt(f)
    }
}
impl Debug for ExactOutputModeIter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "".fmt(f)
    }
}

impl Calculator {
    pub fn next_inexact_output_mode(&mut self) -> CalculationResult {
        let next_mode = self.inexact_output_modes.next().unwrap_or_else(|| {
            self.inexact_output_modes = InexactOutputMode::iter();
            self.inexact_output_modes.next().unwrap()
        });

        let inexact = expression_to_inexact(&self.expression)?;

        match next_mode {
            InexactOutputMode::InexactDegrees => Ok(inexact.into_degrees()),
            InexactOutputMode::InexactRadians => Ok(inexact.into_radians()),
        }
    }

    pub fn build(from: &ErasableCluster) -> Result<Self, ParsingError> {
        let iterator = from.iter();

        Ok(Calculator {
            expression: parse_into_expression(iterator)?,
            inexact_output_modes: InexactOutputMode::iter(),
            exact_output_modes: ExactOutputMode::iter(),
        })
    }
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
