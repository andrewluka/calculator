use std::{iter::Peekable, slice::Iter};

use num_traits::ToPrimitive;

use crate::{
    input_parsing::erasable::{Erasable, ErasableType},
    shared::sign::Sign,
};

use super::{
    calculation_precision::UnsignedValuePrecision,
    calculator::{
        AngleUnit, Expression, Function, MultipliedOrDivided, NamedConstant, NonNamedConstant,
        Term, TermFragment, TermFragmentMagnitude,
    },
    wrapped_iter::WrappedIter,
};

fn integer_as_expression(integer: UnsignedValuePrecision) -> Expression {
    vec![Term {
        fragments: vec![TermFragment {
            fragment_magnitude: TermFragmentMagnitude::NonNamedConstant(NonNamedConstant::Integer(
                integer,
            )),
            multiplied_or_divided: MultipliedOrDivided::default(),
            sign: Sign::default(),
            angle_unit: AngleUnit::Non,
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

                iterator.next();
            }
            ErasableType::DecimalPoint => {
                if was_decimal_point_met {
                    panic!("cannot have more than one decimal point in a number")
                } else {
                    was_decimal_point_met = true;
                }

                iterator.next();
            }
            ErasableType::Formatting => {
                iterator.next();
            }
            _ => break,
        }
    }

    if was_decimal_point_met {
        NonNamedConstant::Decimal {
            before_decimal_point,
            after_decimal_point,
        }
    } else {
        NonNamedConstant::Integer(
            before_decimal_point
                .parse::<UnsignedValuePrecision>()
                .unwrap(),
        )
    }
}

fn parse_term_fragment_operators(
    iterator: &mut Peekable<WrappedIter>,
) -> (Option<Sign>, Option<MultipliedOrDivided>) {
    // figure out whether multiplied or divided
    let multiplied_or_divided = match iterator.peek() {
        Some(erasable) => match erasable {
            Erasable::MultiplicationSign => {
                iterator.next();
                MultipliedOrDivided::Multiplied
            }
            Erasable::DivisionSign => {
                iterator.next();
                MultipliedOrDivided::Divided
            }
            _ => MultipliedOrDivided::Neither,
        },
        None => MultipliedOrDivided::Neither,
    };

    // figure out the sign; any number of plus signs is accepted
    let mut num_of_negative_signs = 0;

    while let Some(erasable) = iterator.peek() {
        match erasable {
            Erasable::PlusSign => {
                iterator.next();
            }
            Erasable::NegativeSign => {
                iterator.next();
                num_of_negative_signs += 1;
            }
            erasable => {
                if let ErasableType::Formatting = (*erasable).into() {
                    iterator.next();
                    continue;
                } else {
                    break;
                }
            }
        }
    }

    let sign = if (num_of_negative_signs % 2) == 0 {
        Sign::Positive
    } else {
        Sign::Negative
    };

    (Some(sign), Some(multiplied_or_divided))
}

fn parse_term_fragment_brackets(iterator: &mut Peekable<WrappedIter>) -> Expression {
    // we'll use it to know when to stop
    let mut bracket_depth: usize = 1;
    let mut inside_the_brackets = Vec::new();

    if let ErasableType::OpeningBracket = iterator.next().unwrap().into() {
    } else {
        panic!("expected opening bracket")
    }

    loop {
        let erasable = iterator.peek().expect("mismatched brackets");
        let erasable_type: ErasableType = (*erasable).into();

        match erasable_type {
            ErasableType::OpeningBracket => {
                inside_the_brackets.push((**erasable).clone());
                iterator.next();

                bracket_depth += 1;
            }
            ErasableType::ClosingBracket => {
                bracket_depth -= 1;

                if bracket_depth == 0 {
                    iterator.next();
                    return parse_into_expression(inside_the_brackets.iter());
                }

                inside_the_brackets.push((**erasable).clone());
                iterator.next();
            }
            _ => {
                inside_the_brackets.push((**erasable).clone());
                iterator.next();
            }
        }
    }
}

fn parse_function_argument_list(iterator: &mut Peekable<WrappedIter>) -> Vec<Expression> {
    let should_be_opening_bracket: ErasableType = iterator.next().unwrap().into();
    assert_eq!(should_be_opening_bracket, ErasableType::OpeningBracket);

    let mut arguments = vec![];

    loop {
        let mut bracket_depth: usize = 1;
        let mut inside_the_bracket = vec![];

        'inner: loop {
            let erasable = *iterator.peek().expect("mismatched brackets");
            let erasable_type: ErasableType = erasable.into();

            match erasable_type {
                ErasableType::OpeningBracket => {
                    bracket_depth += 1;
                    inside_the_bracket.push(erasable.clone());
                    iterator.next();
                }
                ErasableType::ClosingBracket => {
                    // must be first, so the next if expression functions as required
                    bracket_depth -= 1;

                    // now out of argument list
                    if bracket_depth == 0 {
                        iterator.next();
                        // push final argument before returning
                        arguments.push(parse_into_expression(inside_the_bracket.iter()));

                        return arguments;
                    }

                    inside_the_bracket.push(erasable.clone());
                    iterator.next();
                }
                ErasableType::Comma => {
                    // still in argument list
                    if bracket_depth == 1 {
                        iterator.next();
                        arguments.push(parse_into_expression(inside_the_bracket.iter()));
                        break 'inner;
                    }

                    inside_the_bracket.push(erasable.clone());
                    iterator.next();
                }
                _ => {
                    inside_the_bracket.push(erasable.clone());
                    iterator.next();
                }
            }
        }
    }
}

fn parse_function(iterator: &mut Peekable<WrappedIter>) -> TermFragmentMagnitude {
    let function_name = iterator.next().unwrap();
    let args = dbg!(parse_function_argument_list(iterator));
    let mut args = args.into_iter();

    let first_arg = args.next().expect("expected at least one argument");
    assert!(first_arg.len() > 0);

    match function_name {
        Erasable::Absolute => TermFragmentMagnitude::Function(Function::Absolute(first_arg)),
        Erasable::Sin => TermFragmentMagnitude::Function(Function::Sin(first_arg)),
        Erasable::Cos => TermFragmentMagnitude::Function(Function::Cos(first_arg)),
        Erasable::Tan => TermFragmentMagnitude::Function(Function::Tan(first_arg)),
        Erasable::Arcsin => TermFragmentMagnitude::Function(Function::Arcsin(first_arg)),
        Erasable::Arccos => TermFragmentMagnitude::Function(Function::Arccos(first_arg)),
        Erasable::Arctan => TermFragmentMagnitude::Function(Function::Arctan(first_arg)),
        Erasable::NthRoot => {
            let second_arg = args.next().expect("expected 2 arguments");

            TermFragmentMagnitude::Function(Function::NthRoot(first_arg, second_arg))
        }
        _ => panic!("expected function name"),
    }
}

fn parse_term_fragment(
    iterator: &mut Peekable<WrappedIter>,
    sign: Option<Sign>,
    multiplied_or_divided: Option<MultipliedOrDivided>,
) -> Option<TermFragment> {
    if let Some(erasable) = iterator.peek() {
        let erasable_type: ErasableType = (*erasable).into();

        // in case there's an exponent
        let base = match erasable_type {
            ErasableType::ClosingBracket => panic!("unexpected closing bracket"),
            ErasableType::Formatting => {
                iterator.next();
                parse_term_fragment(iterator, sign, multiplied_or_divided)
            }
            ErasableType::ScientificNotation => panic!("unexpected scientific notation"),
            ErasableType::Comma => panic!("unexpected comma"),

            ErasableType::NamedConstant => {
                let result = Some(TermFragment {
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
                    angle_unit: AngleUnit::Non,
                    multiplied_or_divided: MultipliedOrDivided::Multiplied,
                });

                iterator.next();
                result
            }
            ErasableType::AngleUnit => panic!("unexpected angle unit"),
            ErasableType::Digit | ErasableType::DecimalPoint => Some(TermFragment {
                fragment_magnitude: TermFragmentMagnitude::NonNamedConstant(
                    parse_into_int_or_decimal(iterator),
                ),
                multiplied_or_divided: multiplied_or_divided.unwrap_or_default(),
                sign: sign.unwrap_or_default(),
                angle_unit: AngleUnit::Non,
            }),
            ErasableType::ArithmeticOperator => {
                let (sign, multiplied_or_divided) = parse_term_fragment_operators(iterator);
                parse_term_fragment(iterator, sign, multiplied_or_divided)
            }
            ErasableType::FractionDivider => todo!(),
            ErasableType::ExponentPlaceholder => panic!("unexpected exponent"),
            ErasableType::OpeningBracket => Some(TermFragment {
                sign: sign.unwrap_or_default(),
                fragment_magnitude: TermFragmentMagnitude::Bracket(parse_term_fragment_brackets(
                    iterator,
                )),
                angle_unit: AngleUnit::Non,
                multiplied_or_divided: multiplied_or_divided.unwrap_or_default(),
            }),
            ErasableType::FunctionName => Some(TermFragment {
                sign: sign.unwrap_or_default(),
                fragment_magnitude: parse_function(iterator),
                multiplied_or_divided: multiplied_or_divided.unwrap_or_default(),
                angle_unit: AngleUnit::Non,
            }),
        };

        let mut base = base?;

        for _ in 1..=2 {
            if let Some(erasable) = iterator.peek() {
                match erasable {
                    Erasable::ExponentPlaceholder => {
                        iterator.next();
                        let exponent =
                            parse_term_fragment(iterator, None, None).expect("expected exponent");
                        base = TermFragment {
                            fragment_magnitude: TermFragmentMagnitude::NonNamedConstant(
                                NonNamedConstant::Power {
                                    base: vec![Term {
                                        fragments: vec![TermFragment { ..base }],
                                    }],
                                    exponent: vec![Term {
                                        fragments: vec![exponent],
                                    }],
                                },
                            ),
                            // so that if the base is negative the negative sign is not repeated
                            sign: Sign::Positive,
                            ..base
                        };
                    }
                    Erasable::Degrees => {
                        iterator.next();
                        base.angle_unit = AngleUnit::Degrees;
                    }
                    Erasable::Radians => {
                        iterator.next();
                        base.angle_unit = AngleUnit::Radians;
                    }
                    _ => break,
                }
            }
        }

        Some(base)
    } else {
        None
    }
}

fn peek_next_term_fragment(iterator: &mut Peekable<WrappedIter>) -> Option<TermFragment> {
    parse_term_fragment(&mut (iterator.clone()), None, None)
}

fn parse_term(iterator: &mut Peekable<WrappedIter>) -> Option<Term> {
    let mut term = Term { fragments: vec![] };

    let fragment = peek_next_term_fragment(iterator);

    if let Some(fragment) = fragment {
        // signifies the start of a new term
        if let MultipliedOrDivided::Neither = fragment.multiplied_or_divided {
            term.fragments
                .push(parse_term_fragment(iterator, None, None).unwrap());

            while let Some(fragment) = peek_next_term_fragment(iterator) {
                if let MultipliedOrDivided::Neither = fragment.multiplied_or_divided {
                    break;
                }

                println!("{:#?}", fragment);

                term.fragments
                    .push(parse_term_fragment(iterator, None, None).unwrap());
            }

            Some(term)
        } else {
            panic!("expected the start of a new term")
        }
    } else {
        None
    }
}

pub(crate) fn parse_into_expression(iterator: Iter<'_, Erasable>) -> Expression {
    let mut expression = vec![];

    let mut iterator = WrappedIter::from(iterator).peekable();

    while let Some(term) = parse_term(&mut iterator) {
        expression.push(term);
    }

    expression
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::erasable_cluster::ErasableCluster;

    use super::*;

    #[test]
    fn parsing_works() {
        let cluster = ErasableCluster::build("10204.12p").unwrap();

        let expr = parse_into_expression(cluster.iter());
        println!("{:#?}", expr);
    }
}
