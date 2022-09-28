// The purpose of this is to simplify parsing expressions, so the first erasable
// to parse in an expression is always a plus sign.
// This ensures that the resulting term fragment will have MultipliedOrDivided::Neither

use std::slice::Iter;

use crate::input_parsing::erasable::Erasable;

#[derive(Clone)]
pub struct WrappedIter<'a> {
    // we put it at the start of the iteration to simplify
    has_passed_auxiliary_plus_sign: bool,
    inner: Iter<'a, Erasable>,
}

impl<'a> Iterator for WrappedIter<'a> {
    type Item = &'a Erasable;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_passed_auxiliary_plus_sign {
            self.inner.next()
        } else {
            self.has_passed_auxiliary_plus_sign = true;
            Some(&Erasable::PlusSign)
        }
    }
}

impl<'a> From<Iter<'a, Erasable>> for WrappedIter<'a> {
    fn from(iterator: Iter<'a, Erasable>) -> Self {
        Self {
            has_passed_auxiliary_plus_sign: false,
            inner: iterator,
        }
    }
}
