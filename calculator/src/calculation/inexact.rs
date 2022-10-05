// contains Into<Inexact> implementations and Inexact definition

use std::{
    f64::consts::{E, PI},
    ops::Mul,
};

use crate::{input_parsing::erasable::Erasable, shared::errors::CalculationError};

use super::{
    calculation_precision::FloatingPointPrecison,
    calculator::{
        AngleUnit, Expression, Function, MultipliedOrDivided, NamedConstant, Term, TermFragment,
        TermFragmentMagnitude, UnnamedConstant,
    },
    CalculationResult,
};

pub struct Inexact {
    value: FloatingPointPrecison,
    unit: Option<AngleUnit>,
}

impl std::fmt::Display for Inexact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unit = match self.unit {
            Some(unit) => match unit {
                AngleUnit::Degrees => Erasable::Degrees.into(),
                AngleUnit::Radians => Erasable::Radians.into(),
            },
            None => "",
        };
        let value = self.value;

        let separator = if self.unit.is_some() { " " } else { "" };

        format!("{value}{separator}{unit}").fmt(f)
    }
}

impl Inexact {
    pub fn into_radians(self) -> Self {
        if self.unit.is_some() && self.unit.unwrap() == AngleUnit::Degrees {
            Inexact {
                unit: Some(AngleUnit::Radians),
                value: self.value.to_radians(),
            }
        } else {
            self
        }
    }

    pub fn into_degrees(self) -> Self {
        if self.unit.is_some() && self.unit.unwrap() == AngleUnit::Radians {
            Inexact {
                unit: Some(AngleUnit::Degrees),
                value: self.value.to_degrees(),
            }
        } else {
            self
        }
    }

    pub fn is_nan(&self) -> bool {
        self.value.is_nan()
    }
}
impl Mul<Inexact> for Inexact {
    type Output = Inexact;

    fn mul(mut self, mut rhs: Inexact) -> Self::Output {
        self = self.into_radians();
        rhs = rhs.into_radians();

        let unit = if let None = self.unit {
            // if this is None, both would be None so it's fine.
            rhs.unit
        } else {
            self.unit
        };

        Inexact {
            unit,
            value: self.value * rhs.value,
        }
    }
}
impl Mul<FloatingPointPrecison> for Inexact {
    type Output = Inexact;

    fn mul(mut self, rhs: FloatingPointPrecison) -> Self::Output {
        self.value *= rhs;
        self
    }
}
impl core::ops::Add<Inexact> for Inexact {
    type Output = Inexact;

    fn add(mut self, mut rhs: Inexact) -> Self::Output {
        self = self.into_radians();
        rhs = rhs.into_radians();

        Inexact {
            unit: if self.unit.is_some() {
                self.unit
            } else {
                rhs.unit
            },
            value: self.value + rhs.value,
        }
    }
}

impl Into<CalculationResult> for &Term {
    fn into(self) -> CalculationResult {
        let mut result = None;

        for fragment in &self.fragments {
            let inexact: CalculationResult = fragment.into();
            let inexact = inexact?;

            match result {
                Some(product) => result = Some(product * inexact),
                None => result = Some(inexact),
            }
        }

        // match result {
        //     Some(result) => Ok(result),
        //     None => Err(),
        // }

        result.ok_or(CalculationError::new("unexpected empty term".to_string()))
    }
}

impl Into<CalculationResult> for &TermFragment {
    fn into(self) -> CalculationResult {
        let magnitude: CalculationResult = (&self.fragment_magnitude).into();
        let magnitude = magnitude?;

        let multiplier = self.sign as isize as FloatingPointPrecison;

        let mut magnitude = magnitude * multiplier;

        // preserve unit
        magnitude.unit = if magnitude.unit.is_some() {
            magnitude.unit
        } else {
            self.angle_unit
        };

        match self.multiplied_or_divided {
            MultipliedOrDivided::Divided => {
                magnitude.value = 1.0 / magnitude.value;
                Ok(magnitude)
            }
            _ => Ok(magnitude),
        }
    }
}

fn expression_to_radians_if_possible(expression: &Expression) -> CalculationResult {
    let mut angle = expression_to_inexact(&expression)?;

    if angle.unit.is_some() && AngleUnit::Degrees == angle.unit.unwrap() {
        angle.value = angle.value.to_radians();
    }

    Ok(angle)
}
impl Into<CalculationResult> for &TermFragmentMagnitude {
    fn into(self) -> CalculationResult {
        match self {
            TermFragmentMagnitude::Bracket(expression) => expression_to_inexact(&expression),
            TermFragmentMagnitude::Function(function) => function.into(),
            TermFragmentMagnitude::NamedConstant {
                coefficient,
                constant,
            } => {
                let coefficient = expression_to_inexact(&coefficient)?;
                match constant {
                    NamedConstant::E => Ok(coefficient * E),
                    NamedConstant::Pi => Ok(coefficient * PI),
                }
            }
            TermFragmentMagnitude::NonNamedConstant(constant) => match constant {
                UnnamedConstant::Decimal {
                    before_decimal_point,
                    after_decimal_point,
                } => {
                    let value = format!("{}.{}", before_decimal_point, after_decimal_point)
                        .parse::<FloatingPointPrecison>();

                    match value {
                        Ok(value) => Ok(Inexact { value, unit: None }),
                        Err(err) => Err(CalculationError::new(err.to_string())),
                    }
                }
                UnnamedConstant::Fraction {
                    numerator,
                    denominator,
                } => {
                    let numerator = expression_to_radians_if_possible(&numerator)?;
                    let denominator = expression_to_radians_if_possible(&denominator)?;

                    Ok(Inexact {
                        value: numerator.value / denominator.value,
                        unit: numerator.unit,
                    })
                }
                UnnamedConstant::Integer(value) => Ok(Inexact {
                    value: *value as FloatingPointPrecison,
                    unit: None,
                }),
                UnnamedConstant::Power { base, exponent } => {
                    let base = expression_to_inexact(&base)?;
                    let exponent = expression_to_inexact(&exponent)?;

                    let value = base.value.powf(exponent.value);

                    Ok(Inexact {
                        unit: base.unit,
                        value,
                    })
                }
            },
        }
    }
}

impl Into<CalculationResult> for &Function {
    fn into(self) -> CalculationResult {
        match self {
            Function::Absolute(expression) => {
                let mut inexact = expression_to_inexact(&expression)?;
                inexact.value = inexact.value.abs();
                Ok(inexact)
            }
            Function::NthRoot(degree, under_the_root) => {
                let degree = expression_to_inexact(&degree)?;
                let under_the_root = expression_to_inexact(&under_the_root)?;

                Ok(Inexact {
                    unit: under_the_root.unit,
                    value: under_the_root.value.powf(1.0 / degree.value),
                })
            }
            Function::Sin(expression) => Ok(Inexact {
                unit: None,
                value: expression_to_radians_if_possible(&expression)?.value.sin(),
            }),
            Function::Cos(expression) => Ok(Inexact {
                unit: None,
                value: expression_to_radians_if_possible(&expression)?.value.cos(),
            }),
            Function::Tan(expression) => Ok(Inexact {
                unit: None,
                value: expression_to_radians_if_possible(&expression)?.value.tan(),
            }),
            Function::Arcsin(expression) => Ok(Inexact {
                unit: Some(AngleUnit::Radians),
                value: expression_to_inexact(&expression)?.value.asin(),
            }),
            Function::Arccos(expression) => Ok(Inexact {
                unit: Some(AngleUnit::Radians),
                value: expression_to_inexact(&expression)?.value.acos(),
            }),
            Function::Arctan(expression) => Ok(Inexact {
                unit: Some(AngleUnit::Radians),
                value: expression_to_inexact(&expression)?.value.atan(),
            }),
        }
    }
}

pub(crate) fn expression_to_inexact(expression: &Expression) -> CalculationResult {
    let mut sum = None;

    if expression.is_empty() {
        return Err(CalculationError::new("empty expression".to_string()));
    }

    for term in expression {
        let term: CalculationResult = term.into();
        let term = term?;

        match sum {
            Some(prev) => sum = Some(prev + term),
            None => sum = Some(term),
        }
    }

    sum.ok_or(CalculationError::new(
        "unexpected empty expression".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::{
        calculation::calculator::{AngleUnit, Calculator},
        input_parsing::erasable_cluster::ErasableCluster,
    };

    #[test]
    fn expression_to_inexact_works() {
        let cluster = ErasableCluster::build("t(45d) + S(0.5)^4 - 2a(-3)(8)^(2-1 +1)").unwrap();
        let mut calc = Calculator::build(&cluster).unwrap();

        let result = calc.next_inexact_output_mode().unwrap();
        // got that from the internet
        let expected = -382.924838664;

        assert!((expected - result.value).abs() < 1e-4);
        // radians cuz sine returns sine
        assert_eq!(result.unit, Some(AngleUnit::Radians));

        // this time in degrees

        let result = calc.next_inexact_output_mode().unwrap();
        // got that from the internet
        let expected = -21939.9771;

        assert!((expected - result.value).abs() < 1e-4);
        // radians cuz sine returns sine
        assert_eq!(result.unit, Some(AngleUnit::Degrees));
    }
}
