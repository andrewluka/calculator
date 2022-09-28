use std::{collections::HashSet, iter::Peekable, slice::Iter};

use num_traits::ToPrimitive;

use crate::{
    input_parsing::{
        erasable::{Erasable, ErasableType},
        erasable_cluster::ErasableCluster,
    },
    shared::{calculation_precision::UnsignedValuePrecision, sign::Sign},
};

use super::wrapped_iter::WrappedIter;

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
        coefficient: Expression,
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

#[derive(Default)]
enum MultipliedOrDivided {
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

fn integer_as_expression(integer: UnsignedValuePrecision) -> Expression {
    vec![Term {
        fragments: vec![TermFragment {
            fragment_magnitude: TermFragmentMagnitude::NonNamedConstant(NonNamedConstant::Integer(
                integer,
            )),
            multiplied_or_divided: MultipliedOrDivided::default(),
            sign: Sign::default(),
        }],
    }]
}

fn parse_into_int_or_decimal(iterator: &mut Peekable<WrappedIter>) -> NonNamedConstant {
    let mut was_decimal_point_met = false;
    let mut before_decimal_point = String::new();
    let mut after_decimal_point = String::new();

    loop {
        let erasable = match iterator.peek() {
            Some(e) => *e,
            None => break,
        };
        let erasable_type: ErasableType = erasable.into();

        match erasable_type {
            ErasableType::Digit => {
                let digit = <Erasable as ToPrimitive>::to_u8(erasable).unwrap() as char;

                if was_decimal_point_met {
                    after_decimal_point.push(digit);
                } else {
                    before_decimal_point.push(digit);
                }
            }
            ErasableType::DecimalPoint => {
                if was_decimal_point_met {
                    panic!("cannot have more than one decimal point in a number")
                } else {
                    was_decimal_point_met = true;
                    before_decimal_point.push('.');
                }
            }
            _ => break,
        }

        iterator.next();
    }

    if was_decimal_point_met {
        NonNamedConstant::Decimal {
            before_decimal_point: before_decimal_point.parse().unwrap(),
            after_decimal_point: after_decimal_point.parse().unwrap(),
        }
    } else {
        NonNamedConstant::Integer(before_decimal_point.parse().unwrap())
    }
}

fn parse_term_fragmemt(
    iterator: &mut Peekable<WrappedIter>,
    sign: Option<Sign>,
    multiplied_or_divided: Option<MultipliedOrDivided>,
) -> Option<TermFragment> {
    if let Some(erasable) = iterator.peek() {
        let erasable = *erasable;
        let erasable_type: ErasableType = erasable.into();

        // note before we start: DON'T FORGET TO PARSE POWERS AND ANGLE UNITS

        // when we first see:

        // ClosingBracket: error.

        // Formatting: skip

        // ScientificNotation: error (it cannot be on its own, it must be after a digit)

        // Comma: error (must only be used to separate function arguments)

        // NamedConstant: parse it into its own fragment.

        // AngleUnit: if you see this first, error.

        // Digit: go with it and parse it into an integer/decimal/power, and
        // stop once you see something that is not a digit, or once you see
        // double commas.
        // (Parse any power as an expression)

        // DecimalPoint: parse it the same as Digit.

        // FunctionName: separate it into the required number of expressions,
        // using the opening/closing brackets and commas as guidance.

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

        //
        //

        // in case there's an exponent
        let base = match erasable_type {
            ErasableType::ClosingBracket => panic!("unexpected closing bracket"),
            ErasableType::Formatting => {
                iterator.next();
                parse_term_fragmemt(iterator, None, None)
            }
            ErasableType::ScientificNotation => panic!("unexpected scientific notation"),
            ErasableType::Comma => panic!("unexpected comma"),
            ErasableType::NamedConstant => Some(TermFragment {
                sign: Sign::default(),
                fragment_magnitude: TermFragmentMagnitude::NamedConstant {
                    coefficient: integer_as_expression(1),
                    constant: match erasable {
                        Erasable::Pi => NamedConstant::Pi,
                        Erasable::E => NamedConstant::E,
                        Erasable::I => NamedConstant::I,
                        _ => panic!("unexpected: {:?}", erasable),
                    },
                },
                multiplied_or_divided: MultipliedOrDivided::Multiplied,
            }),
            ErasableType::AngleUnit => panic!("unexpected angle unit"),
            ErasableType::Digit | ErasableType::DecimalPoint => Some(TermFragment {
                fragment_magnitude: TermFragmentMagnitude::NonNamedConstant(
                    parse_into_int_or_decimal(iterator),
                ),
                multiplied_or_divided: multiplied_or_divided.unwrap_or_default(),
                sign: sign.unwrap_or_default(),
            }),
        };

        todo!()
    } else {
        None
    }
}

fn peek_next_term_fragment(iterator: &mut Peekable<WrappedIter>) -> Option<TermFragment> {
    parse_term_fragmemt(&mut (iterator.clone()), None, None)
}

fn parse_term(iterator: &mut Peekable<WrappedIter>) -> Option<Term> {
    let mut term = Term { fragments: vec![] };

    let fragment = peek_next_term_fragment(iterator);

    if let Some(fragment) = fragment {
        // signifies the start of a new term
        if let MultipliedOrDivided::Neither = fragment.multiplied_or_divided {
            term.fragments
                .push(parse_term_fragmemt(iterator, None, None).unwrap());

            while let Some(fragment) = peek_next_term_fragment(iterator) {
                if let MultipliedOrDivided::Neither = fragment.multiplied_or_divided {
                    break;
                }

                term.fragments
                    .push(parse_term_fragmemt(iterator, None, None).unwrap());
            }

            Some(term)
        } else {
            panic!("expected the start of a new term")
        }
    } else {
        None
    }
}

fn parse_into_expression(iterator: Iter<'_, Erasable>) -> Expression {
    let mut expression = vec![];

    let mut iterator = WrappedIter::from(iterator).peekable();

    while let Some(term) = parse_term(&mut iterator) {
        expression.push(term);
    }

    expression
}

impl From<&ErasableCluster> for ParsingCalculator {
    fn from(cluster: &ErasableCluster) -> Self {
        let mut iterator = cluster.iter();

        ParsingCalculator {
            expression: parse_into_expression(iterator),
            output_modes: HashSet::new(),
        }
    }
}
