use crate::shared::errors::CalculationError;

use self::inexact::Inexact;

pub(crate) mod calculation_precision;
pub mod calculator;
pub(self) mod helpers;
pub(self) mod inexact;
pub(self) mod parsers;
pub(self) mod rational_number;
pub(self) mod wrapped_iter;

pub(self) type CalculationResult = Result<Inexact, CalculationError>;
