// contains Into<Inexact> implementations and Inexact definition

use std::{
    f64::consts::{E, PI},
    ops::Mul,
};

use crate::input_parsing::erasable::Erasable;

use super::{
    calculation_precision::FloatingPointPrecison,
    calculator::{
        AngleUnit, Expression, Function, MultipliedOrDivided, NamedConstant, Term, TermFragment,
        TermFragmentMagnitude, UnnamedConstant,
    },
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

impl Into<Inexact> for &Term {
    fn into(self) -> Inexact {
        self.fragments
            .iter()
            .map(|fragment| <&TermFragment as Into<Inexact>>::into(fragment))
            .reduce(|a, b| a * b)
            .unwrap()
    }
}

impl Into<Inexact> for &TermFragment {
    fn into(self) -> Inexact {
        let magnitude: Inexact = (&self.fragment_magnitude).into();
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
                magnitude
            }
            _ => magnitude,
        }
    }
}

fn expression_to_radians_if_possible(expression: &Expression) -> Inexact {
    let mut angle = expression_to_inexact(&expression);

    if angle.unit.is_some() && AngleUnit::Degrees == angle.unit.unwrap() {
        angle.value = angle.value.to_radians();
    }

    angle
}
impl Into<Inexact> for &TermFragmentMagnitude {
    fn into(self) -> Inexact {
        match self {
            TermFragmentMagnitude::Bracket(expression) => expression_to_inexact(&expression),
            TermFragmentMagnitude::Function(function) => function.into(),
            TermFragmentMagnitude::NamedConstant {
                coefficient,
                constant,
            } => {
                let coefficient = expression_to_inexact(&coefficient);
                match constant {
                    NamedConstant::E => coefficient * E,
                    NamedConstant::Pi => coefficient * PI,
                }
            }
            TermFragmentMagnitude::NonNamedConstant(constant) => match constant {
                UnnamedConstant::Decimal {
                    before_decimal_point,
                    after_decimal_point,
                } => Inexact {
                    value: format!("{}.{}", before_decimal_point, after_decimal_point)
                        .parse()
                        .unwrap(),
                    unit: None,
                },
                UnnamedConstant::Fraction {
                    numerator,
                    denominator,
                } => {
                    let numerator = expression_to_radians_if_possible(&numerator);
                    let denominator = expression_to_radians_if_possible(&denominator);

                    Inexact {
                        value: numerator.value / denominator.value,
                        unit: numerator.unit,
                    }
                }
                UnnamedConstant::Integer(value) => Inexact {
                    value: *value as FloatingPointPrecison,
                    unit: None,
                },
                UnnamedConstant::Power { base, exponent } => {
                    let base = expression_to_inexact(&base);
                    let exponent = expression_to_inexact(&exponent);

                    let value = base.value.powf(exponent.value);

                    Inexact {
                        unit: base.unit,
                        value,
                    }
                }
            },
        }
    }
}

impl Into<Inexact> for &Function {
    fn into(self) -> Inexact {
        // match self {
        //     Self::NthRoot(n, value_under_root) => {}
        // }
        match self {
            Function::Absolute(expression) => {
                let mut inexact = expression_to_inexact(&expression);
                inexact.value = inexact.value.abs();
                inexact
            }
            Function::NthRoot(degree, under_the_root) => {
                let degree = expression_to_inexact(&degree);
                let under_the_root = expression_to_inexact(&under_the_root);

                Inexact {
                    unit: under_the_root.unit,
                    value: under_the_root.value.powf(1.0 / degree.value),
                }
            }
            Function::Sin(expression) => Inexact {
                unit: None,
                value: expression_to_radians_if_possible(&expression).value.sin(),
            },
            Function::Cos(expression) => Inexact {
                unit: None,
                value: expression_to_radians_if_possible(&expression).value.cos(),
            },
            Function::Tan(expression) => Inexact {
                unit: None,
                value: dbg!(dbg!(expression_to_radians_if_possible(&expression).value).tan()),
            },
            Function::Arcsin(expression) => Inexact {
                unit: Some(AngleUnit::Radians),
                value: expression_to_inexact(&expression).value.asin(),
            },
            Function::Arccos(expression) => Inexact {
                unit: Some(AngleUnit::Radians),
                value: expression_to_inexact(&expression).value.acos(),
            },
            Function::Arctan(expression) => Inexact {
                unit: Some(AngleUnit::Radians),
                value: expression_to_inexact(&expression).value.atan(),
            },
        }
    }
}

pub(crate) fn expression_to_inexact(expression: &Expression) -> Inexact {
    expression
        .iter()
        .map(|term| term.into())
        .reduce(|a: Inexact, b: Inexact| a + b)
        .unwrap()
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
        let mut calc = Calculator::from(&cluster);

        let result = calc.next_inexact_output_mode();
        // got that from the internet
        let expected = -382.924838664;

        assert!((expected - result.value).abs() < 1e-4);
        // radians cuz sine returns sine
        assert_eq!(result.unit, Some(AngleUnit::Radians));

        // this time in degrees

        let result = calc.next_inexact_output_mode();
        // got that from the internet
        let expected = -21939.9771;

        assert!((expected - result.value).abs() < 1e-4);
        // radians cuz sine returns sine
        assert_eq!(result.unit, Some(AngleUnit::Degrees));
    }
}
