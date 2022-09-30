use std::collections::HashSet;

use crate::{
    input_parsing::erasable_cluster::ErasableCluster,
    shared::{calculation_precision::UnsignedValuePrecision, sign::Sign},
};

use super::parsers::parse_into_expression;

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
    pub(super) angle_unit: AngleUnit,
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

#[derive(Default, Clone, Copy, Debug)]
pub(super) enum MultipliedOrDivided {
    #[default]
    Multiplied,
    Divided,

    // used to signify the beginning of a term
    Neither,
}

// TODO: ParsingCalculator

enum OutputModes {
    ExactImproperFractionRadians,
    ExactImproperFractionDegrees,
    ExactMixedFractionRadians,
    ExactMixedFractionDegrees,
    DecimalRadians,
    DecimalDegrees,
}

pub struct ParsingCalculator {
    expression: Expression,
    output_modes: HashSet<OutputModes>,
}

impl From<&ErasableCluster> for ParsingCalculator {
    fn from(cluster: &ErasableCluster) -> Self {
        let iterator = cluster.iter();

        ParsingCalculator {
            expression: parse_into_expression(iterator),
            output_modes: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() {
        let cluster = ErasableCluster::build("1").unwrap();

        let calc = ParsingCalculator::from(&cluster);
    }
}
