use std::{iter::Peekable, slice::Iter};

use num_traits::ToPrimitive;

use crate::{
    input_parsing::erasable::{Erasable, ErasableType},
    shared::{errors::ParsingError, sign::Sign},
};

use super::{
    calculation_precision::UnsignedValuePrecision,
    calculator::{
        AngleUnit, Expression, Function, MultipliedOrDivided, NamedConstant, Term, TermFragment,
        TermFragmentMagnitude, UnnamedConstant,
    },
    wrapped_iter::WrappedIter,
};

enum ParsingResult<T> {
    Some(T),
    None,
    Err(ParsingError),
}
macro_rules! some_from_parsing_result_or_return {
    ($value:expr) => {{
        let val = match $value {
            ParsingResult::Some(v) => v,
            ParsingResult::None => return ParsingResult::None,
            ParsingResult::Err(e) => return ParsingResult::Err(e),
        };
        val
    }};
}
macro_rules! some_from_option_or_will_error {
    ($value:expr) => {{
        let val = match $value {
            Some(v) => v,
            None => return ParsingResult::Err(ParsingError::EndOfInput),
        };

        val
    }};
}
macro_rules! some_from_result {
    ($value:expr) => {{
        match $value {
            Ok(v) => v,
            Err(e) => return ParsingResult::Err(ParsingError::Custom(e.to_string())),
        }
    }};
}
macro_rules! some_from_parsing_result_or_will_error {
    ($value:expr) => {{
        let val = match $value {
            ParsingResult::Some(v) => v,
            ParsingResult::None => return ParsingResult::Err(ParsingError::EndOfInput),
            ParsingResult::Err(e) => return ParsingResult::Err(e),
        };
        val
    }};
}

fn integer_as_expression(integer: UnsignedValuePrecision) -> Expression {
    vec![Term {
        fragments: vec![TermFragment {
            fragment_magnitude: TermFragmentMagnitude::NonNamedConstant(UnnamedConstant::Integer(
                integer,
            )),
            multiplied_or_divided: MultipliedOrDivided::default(),
            sign: Sign::default(),
            angle_unit: None,
        }],
    }]
}

fn parse_into_int_or_decimal(
    iterator: &mut Peekable<WrappedIter>,
) -> ParsingResult<UnnamedConstant> {
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
                let digit = <Erasable as ToPrimitive>::to_u8(erasable);
                let digit = some_from_option_or_will_error!(digit) as char;

                if was_decimal_point_met {
                    after_decimal_point.push(digit);
                } else {
                    before_decimal_point.push(digit);
                }

                iterator.next();
            }
            ErasableType::DecimalPoint => {
                if was_decimal_point_met {
                    return ParsingResult::Err(ParsingError::ExcessiveDecimalPoints);
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
        ParsingResult::Some(UnnamedConstant::Decimal {
            before_decimal_point,
            after_decimal_point,
        })
    } else {
        let i = before_decimal_point.parse::<UnsignedValuePrecision>();
        let i = some_from_result!(i);

        ParsingResult::Some(UnnamedConstant::Integer(i))
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

fn parse_term_fragment_brackets(iterator: &mut Peekable<WrappedIter>) -> ParsingResult<Expression> {
    // we'll use it to know when to stop
    let mut bracket_depth: usize = 1;
    let mut inside_the_brackets = Vec::new();

    let should_be_bracket = iterator.next();

    match should_be_bracket {
        Some(should_be_bracket) => {
            if <&Erasable as Into<ErasableType>>::into(should_be_bracket)
                != ErasableType::OpeningBracket
            {
                return ParsingResult::Err(ParsingError::ExpectedButFound {
                    expected: String::from("opening bracket"),
                    found: should_be_bracket.to_string(),
                });
            }
        }
        None => return ParsingResult::Err(ParsingError::EndOfInput),
    }

    loop {
        let erasable = iterator.peek();

        if let None = erasable {
            return ParsingResult::Err(ParsingError::MismatchedBrackets);
        }

        let erasable = some_from_option_or_will_error!(erasable);
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

                    let inside_the_brackets = parse_into_expression(inside_the_brackets.iter());
                    return ParsingResult::Some(some_from_result!(inside_the_brackets));
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

fn parse_function_argument_list(
    iterator: &mut Peekable<WrappedIter>,
) -> ParsingResult<Vec<Expression>> {
    let should_be_opening_bracket: ErasableType =
        some_from_option_or_will_error!(iterator.next()).into();

    if should_be_opening_bracket != ErasableType::OpeningBracket {
        return ParsingResult::Err(ParsingError::Unexpected("opening bracket".to_string()));
    }

    let mut arguments: Vec<Expression> = vec![];

    loop {
        let mut bracket_depth: usize = 1;
        let mut inside_the_bracket = vec![];

        'inner: loop {
            let erasable = iterator.peek();

            if erasable == None {
                return ParsingResult::Err(ParsingError::MismatchedBrackets);
            }

            let erasable = *some_from_option_or_will_error!(erasable);
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
                        let arg = parse_into_expression(inside_the_bracket.iter());

                        arguments.push(some_from_result!(arg));
                        return ParsingResult::Some(arguments);
                    }

                    inside_the_bracket.push(erasable.clone());
                    iterator.next();
                }
                ErasableType::Comma => {
                    // still in argument list
                    if bracket_depth == 1 {
                        iterator.next();
                        let arg = parse_into_expression(inside_the_bracket.iter());
                        arguments.push(some_from_result!(arg));
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

fn parse_function(iterator: &mut Peekable<WrappedIter>) -> ParsingResult<TermFragmentMagnitude> {
    let function_name = iterator.next();
    let function_name = some_from_option_or_will_error!(function_name);

    let args = parse_function_argument_list(iterator);
    let args = some_from_parsing_result_or_return!(args);
    let mut args = args.into_iter();

    let first_arg = args.next();
    let first_arg = some_from_option_or_will_error!(first_arg);

    let f = match function_name {
        Erasable::Absolute => TermFragmentMagnitude::Function(Function::Absolute(first_arg)),
        Erasable::Sin => TermFragmentMagnitude::Function(Function::Sin(first_arg)),
        Erasable::Cos => TermFragmentMagnitude::Function(Function::Cos(first_arg)),
        Erasable::Tan => TermFragmentMagnitude::Function(Function::Tan(first_arg)),
        Erasable::Arcsin => TermFragmentMagnitude::Function(Function::Arcsin(first_arg)),
        Erasable::Arccos => TermFragmentMagnitude::Function(Function::Arccos(first_arg)),
        Erasable::Arctan => TermFragmentMagnitude::Function(Function::Arctan(first_arg)),
        Erasable::NthRoot => {
            let second_arg = args.next();
            let second_arg = some_from_option_or_will_error!(second_arg);

            TermFragmentMagnitude::Function(Function::NthRoot(first_arg, second_arg))
        }
        _ => {
            return ParsingResult::Err(ParsingError::ExpectedButFound {
                expected: String::from("function name"),
                found: function_name.to_string(),
            })
        }
    };

    ParsingResult::Some(f)
}

fn parse_term_fragment(
    iterator: &mut Peekable<WrappedIter>,
    sign: Option<Sign>,
    multiplied_or_divided: Option<MultipliedOrDivided>,
) -> ParsingResult<TermFragment> {
    if let Some(erasable) = iterator.peek() {
        let erasable_type: ErasableType = (*erasable).into();

        // in case there's an exponent
        let mut base: TermFragment = match erasable_type {
            ErasableType::ClosingBracket => {
                return ParsingResult::Err(
                    ParsingError::CannotParseEmptyString, // "unexpected closing bracket"
                );
            }
            ErasableType::Formatting => {
                iterator.next();
                let frag = parse_term_fragment(iterator, sign, multiplied_or_divided);

                some_from_parsing_result_or_return!(frag)
            }
            ErasableType::ScientificNotation => {
                return ParsingResult::Err(ParsingError::Unexpected(erasable.to_string()))
            }
            ErasableType::Comma => {
                return ParsingResult::Err(ParsingError::Unexpected(erasable.to_string()))
            }
            ErasableType::NamedConstant => {
                let result = TermFragment {
                    sign: sign.unwrap_or_default(),
                    fragment_magnitude: TermFragmentMagnitude::NamedConstant {
                        coefficient: integer_as_expression(1),
                        constant: match erasable {
                            Erasable::Pi => NamedConstant::Pi,
                            Erasable::E => NamedConstant::E,
                            _ => {
                                return ParsingResult::Err(ParsingError::Unexpected(
                                    erasable.to_string(),
                                ))
                            }
                        },
                    },
                    angle_unit: None,
                    multiplied_or_divided: multiplied_or_divided.unwrap_or_default(),
                };

                iterator.next();
                result
            }
            ErasableType::AngleUnit => {
                return ParsingResult::Err(ParsingError::Unexpected(erasable.to_string()))
            }
            ErasableType::Digit | ErasableType::DecimalPoint => TermFragment {
                fragment_magnitude: TermFragmentMagnitude::NonNamedConstant(
                    some_from_parsing_result_or_return!(parse_into_int_or_decimal(iterator)),
                ),
                multiplied_or_divided: multiplied_or_divided.unwrap_or_default(),
                sign: sign.unwrap_or_default(),
                angle_unit: None,
            },
            ErasableType::ArithmeticOperator => {
                let (sign, multiplied_or_divided) = parse_term_fragment_operators(iterator);
                some_from_parsing_result_or_return!(parse_term_fragment(
                    iterator,
                    sign,
                    multiplied_or_divided
                ))
            }
            ErasableType::FractionDivider => todo!(),
            ErasableType::ExponentPlaceholder => {
                return ParsingResult::Err(ParsingError::Unexpected(erasable.to_string()))
            }
            ErasableType::OpeningBracket => TermFragment {
                sign: sign.unwrap_or_default(),
                fragment_magnitude: TermFragmentMagnitude::Bracket(
                    some_from_parsing_result_or_return!(parse_term_fragment_brackets(iterator)),
                ),
                angle_unit: None,
                multiplied_or_divided: multiplied_or_divided.unwrap_or_default(),
            },
            ErasableType::FunctionName => TermFragment {
                sign: sign.unwrap_or_default(),
                fragment_magnitude: some_from_parsing_result_or_return!(parse_function(iterator)),
                multiplied_or_divided: multiplied_or_divided.unwrap_or_default(),
                angle_unit: None,
            },
        };

        for _ in 1..=2 {
            if let Some(erasable) = iterator.peek() {
                match erasable {
                    Erasable::ExponentPlaceholder => {
                        iterator.next();
                        let exponent = some_from_parsing_result_or_return!(parse_term_fragment(
                            iterator, None, None
                        ));
                        base = TermFragment {
                            fragment_magnitude: TermFragmentMagnitude::NonNamedConstant(
                                UnnamedConstant::Power {
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
                        base.angle_unit = Some(AngleUnit::Degrees);
                    }
                    Erasable::Radians => {
                        iterator.next();
                        base.angle_unit = Some(AngleUnit::Radians);
                    }
                    _ => break,
                }
            }
        }

        ParsingResult::Some(base)
    } else {
        ParsingResult::None
    }
}

fn peek_next_term_fragment(iterator: &mut Peekable<WrappedIter>) -> ParsingResult<TermFragment> {
    parse_term_fragment(&mut (iterator.clone()), None, None)
}

fn parse_term(iterator: &mut Peekable<WrappedIter>) -> ParsingResult<Term> {
    let mut term = Term { fragments: vec![] };

    let fragment = peek_next_term_fragment(iterator);
    let fragment = some_from_parsing_result_or_return!(fragment);

    // signifies the start of a new term
    if let MultipliedOrDivided::Neither = fragment.multiplied_or_divided {
        let fragment = parse_term_fragment(iterator, None, None);
        let fragment = some_from_parsing_result_or_return!(fragment);
        term.fragments.push(fragment);

        loop {
            match peek_next_term_fragment(iterator) {
                ParsingResult::Err(e) => return ParsingResult::Err(e),
                ParsingResult::Some(fragment) => {
                    if let MultipliedOrDivided::Neither = fragment.multiplied_or_divided {
                        break;
                    }

                    term.fragments.push(some_from_parsing_result_or_will_error!(
                        parse_term_fragment(iterator, None, None)
                    ));
                }
                ParsingResult::None => break,
            }
        }

        ParsingResult::Some(term)
    } else {
        ParsingResult::Err(ParsingError::Custom(
            "expected the start of a new term".to_string(),
        ))
    }
}

pub(crate) fn parse_into_expression(
    iterator: Iter<'_, Erasable>,
) -> Result<Expression, ParsingError> {
    let mut expression = vec![];

    let mut iterator = WrappedIter::from(iterator).peekable();

    loop {
        let term = parse_term(&mut iterator);

        match term {
            ParsingResult::Some(term) => expression.push(term),
            ParsingResult::Err(e) => return Err(e),
            ParsingResult::None => break,
        }
    }

    Ok(expression)
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::erasable_cluster::ErasableCluster;

    use super::*;

    #[test]
    fn parsing_works() {
        let cluster = ErasableCluster::build("10204.12p").unwrap();

        let expr = parse_into_expression(cluster.iter());
    }
}
