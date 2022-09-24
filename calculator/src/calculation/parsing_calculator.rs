use crate::shared::{calculation_precision::UnsignedValuePrecision, sign::Sign};

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
    Neither,
}

// TODO: ParsingCalculator
