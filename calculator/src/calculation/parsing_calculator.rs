use std::{collections::HashSet, iter::Peekable, slice::Iter};

use crate::{
    input_parsing::{
        erasable::{Erasable, ErasableType},
        erasable_cluster::ErasableCluster,
    },
    shared::{calculation_precision::UnsignedValuePrecision, sign::Sign},
};

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

enum Angle {
    Degrees(Expression),
    Radians(Expression),
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
        fn parse_term_fragmemt(
            iterator: &mut Peekable<Iter<'_, Erasable>>,
        ) -> Option<TermFragment> {
            if let Some(erasable) = iterator.peek() {
                let erasable = *erasable;
                let erasable_type: ErasableType = erasable.into();

                // note before we start: DON'T FORGET TO PARSE POWERS AND ANGLE UNITS

                // when we first see:
                // Digit: go with it and parse it into an integer/decimal/power, and
                // stop once you see something that is not a digit, or once you see
                // double commas.
                // (Parse any power as an expression)

                // ArithmeticOperator: that's gonna be the multiplied/divided and sign
                // part. However, the multiplication/division operator must precede the
                // sign operator (otherwise error).
                // The multiplication/division operator is optional; default Neither
                // (beginning of a new term).
                // Several addition/subtraction operators are permitted to go after each
                // other. After the operators, parse the remainder as a new term fragment.

                // Opening bracket: go with it until you find a matching number of opening and
                // closing brackets, then parse those into an expression.
                // Careful in case there's a caret there.

                // ClosingBracket: error.

                // Formatting: skip

                // DecimalPoint: parse it the same as Digit.

                // ScientificNotation: error (it cannot be on its own, it must be after a digit)

                // Comma: error (must only be used to separate function arguments)

                // NamedConstant: parse it into its own fragment.

                // FunctionName: separate it into the required number of expressions,
                // using the opening/closing brackets and commas as guidance.

                // AngleUnit: if you see this first, error.

                //
                //

                todo!()
            } else {
                None
            }
        }

        fn parse_term(iterator: &mut Peekable<Iter<'_, Erasable>>) -> Option<Term> {
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

        fn parse_into_expression(iterator: &mut Peekable<Iter<'_, Erasable>>) -> Expression {
            let mut expression = vec![];

            while let Some(term) = parse_term(iterator) {
                expression.push(term);
            }

            expression
        }

        let mut iterator = cluster.iter().peekable();

        ParsingCalculator {
            expression: parse_into_expression(&mut iterator),
            output_modes: HashSet::new(),
        }
    }
}
