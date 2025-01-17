//! Compile-time guards and static assertions.
//!
//! ## Overview
//! Most type operators in this module copies the input type to `Self::Output`
//! when certain conditions are met. We have these categories of operators:
//!
//! - [IfOutput<Output, Type>](crate::control::IfOutput):
//!     Asserts `Type` can be constructed.
//! - [IfPredicate<Output, Predicate>](crate::control::IfPredicate):
//!     Asserts `Predicate` derives to [True](typenum::True).
//! - [IfSame<Output, Lhs, Rhs>](crate::control::IfSame):
//!     Asserts `Lhs` and `Rhs` are of the same type.
//! - [IfLess](crate::control::IfLess), [IfLessOrEqual](crate::control::IfLessOrEqual),
//!     [IfGreater](crate::control::IfGreater), [IfGreaterOrEqual](crate::control::IfGreaterOrEqual),
//!     [IfEqual](crate::control::IfEqual):
//!     Asserts two [typenum] numbers follows the order.
//!
//! By convention, [IfSameOutput<Output, Lhs, Rhs>](crate::control::IfSameOutput) is type alias of
//! `<Output as IfSame<Lhs, Rhs>>::Output` trait cast, and others follow.
//! Only [IfOutput<Output, Type>](crate::control::IfOutput) has no corresponding trait.
//!
//! ## Static assertions
//! We can make use of `If*Output` aliases to build compile time assertions.
//! For example, [IfLessOutput](crate::control::IfLessOutput) asserts LHS
//! is less than RHS.
//!
//! ```ignore
//! use typenum::consts::*;
//! use type_freak::control::IfLessOutput;
//!
//! type Out1 = IfLessOutput<usize, U3, U5>;  // U3 < U5 is true, thus Out1 ~= usize
//! type Out2 = IfLessOutput<usize, U5, U3>;  // U5 < U5 is false
//!
//! fn assert() {
//!    let _: Out1 = 0;  // Goes fine here.
//!    let _: Out2 = 0;  // Compile error!!!
//! }
//!  ```
//!
//! ## Compile-time guards
//! By placing `If*` trait bounds in `where` block. we can build compile-time
//! guarded functions. For example, we add `IfSame` trait bound to assert two function
//! generic parameters have identical types.
//! The same trick applies to guarded structs, traits and impl blocks.
//!
//! ```ignore
//! use type_freak::control::IfSame;
//!
//! fn guarded_function<Lhs, Rhs>() -> String
//! where
//!     Lhs: IfSame<Lhs, Rhs>
//! {
//!     "Yeeeeeee!".to_owned()
//! }
//!
//! fn comile_me() {
//!     let _ = guarded_function::<String, String>();  // fine
//!     let _ = guarded_function::<String, u8>();      // Compile error!!!
//! }
//! ```
//!
//!

use crate::{boolean::Boolean, tuple::FirstOfOutput};
use typenum::{
    Eq, False, Gr, GrEq, IsEqual, IsGreater, IsGreaterOrEqual, IsLess, IsLessOrEqual, Le, LeEq,
    True,
};

// if

/// A type alias that checks if type can be constructed.
pub type IfOutput<Output, Cond> = FirstOfOutput<(Output, Cond)>;

// if type equivalence

/// A type operator that checks you both types are equivalent.
pub trait IfSame<Lhs, Rhs> {
    type Output;
}

pub type IfSameOutput<Output, Lhs, Rhs> = <Output as IfSame<Lhs, Rhs>>::Output;

impl<Same, Output> IfSame<Same, Same> for Output {
    type Output = Output;
}

// if predicate

/// A type operator that checks if condition is [True](crate::boolean::True).
pub trait IfPredicate<Cond>
where
    Cond: Boolean,
{
    type Output;
}

pub type IfPredicateOutput<Output, Cond> = <Output as IfPredicate<Cond>>::Output;

impl<Output> IfPredicate<True> for Output {
    type Output = Output;
}

// if-else predicate

/// A type operator that returns output depending [Boolean](crate::boolean::Boolean) condition.
pub trait IfElsePredicate<Cond>
where
    Cond: Boolean,
{
    type Output;
}

pub type IfElsePredicateOutput<TrueOutput, FalseOutput, Cond> =
    <(TrueOutput, FalseOutput) as IfElsePredicate<Cond>>::Output;

impl<TrueOutput, FalseOutput> IfElsePredicate<True> for (TrueOutput, FalseOutput) {
    type Output = TrueOutput;
}

impl<TrueOutput, FalseOutput> IfElsePredicate<False> for (TrueOutput, FalseOutput) {
    type Output = FalseOutput;
}

// if not predicate

/// A type operator that checks if condition is [False](crate::boolean::False).
pub trait IfNotPredicate<Cond>
where
    Cond: Boolean,
{
    type Output;
}

pub type IfNotPredicateOutput<Output, Cond> = <Output as IfPredicate<Cond>>::Output;

impl<Output> IfNotPredicate<False> for Output {
    type Output = Output;
}

// if less than

/// A type operator that checks if left-hand-site is less than right-hand-side.
pub trait IfLess<Lhs, Rhs> {
    type Output;
}

pub type IfLessOutput<Output, Lhs, Rhs> = <Output as IfLess<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfLess<Lhs, Rhs> for Output
where
    Lhs: IsLess<Rhs>,
    Output: IfPredicate<Le<Lhs, Rhs>>,
    Le<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, Le<Lhs, Rhs>>;
}

// if less than or equal

/// A type operator that checks if left-hand-site is less than or equals to right-hand-side.
pub trait IfLessOrEqual<Lhs, Rhs> {
    type Output;
}

pub type IfLessOrEqualOutput<Output, Lhs, Rhs> = <Output as IfLessOrEqual<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfLessOrEqual<Lhs, Rhs> for Output
where
    Lhs: IsLessOrEqual<Rhs>,
    Output: IfPredicate<LeEq<Lhs, Rhs>>,
    LeEq<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, LeEq<Lhs, Rhs>>;
}

// if greater than

/// A type operator that checks if left-hand-site is greater than right-hand-side.
pub trait IfGreater<Lhs, Rhs> {
    type Output;
}

pub type IfGreaterOutput<Output, Lhs, Rhs> = <Output as IfGreater<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfGreater<Lhs, Rhs> for Output
where
    Lhs: IsGreater<Rhs>,
    Output: IfPredicate<Gr<Lhs, Rhs>>,
    Gr<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, Gr<Lhs, Rhs>>;
}

// if greater than or equal

/// A type operator that checks if left-hand-site is greater than or equals to right-hand-side.
pub trait IfGreaterOrEqual<Lhs, Rhs> {
    type Output;
}

pub type IfGreaterOrEqualOutput<Output, Lhs, Rhs> = <Output as IfGreaterOrEqual<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfGreaterOrEqual<Lhs, Rhs> for Output
where
    Lhs: IsGreaterOrEqual<Rhs>,
    Output: IfPredicate<GrEq<Lhs, Rhs>>,
    GrEq<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, GrEq<Lhs, Rhs>>;
}

// if equal

/// A type operator that checks if left-hand-site equals to right-hand-side.
pub trait IfEqual<Lhs, Rhs> {
    type Output;
}

pub type IfEqualOutput<Output, Lhs, Rhs> = <Output as IfEqual<Lhs, Rhs>>::Output;

impl<Lhs, Rhs, Output> IfEqual<Lhs, Rhs> for Output
where
    Lhs: IsEqual<Rhs>,
    Output: IfPredicate<Eq<Lhs, Rhs>>,
    Eq<Lhs, Rhs>: Boolean,
{
    type Output = IfPredicateOutput<Output, Eq<Lhs, Rhs>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{consts::*, Le, Unsigned};

    type Assert1 = IfOutput<U3, ()>;
    type Assert2 = IfSameOutput<(), u8, u8>;
    type Assert3 = IfPredicateOutput<(), Le<U3, U4>>;
    type Assert4 = IfElsePredicateOutput<True, False, Le<U3, U4>>;

    type Assert5 = IfLessOutput<(), U6, U9>;

    type Assert6 = IfLessOrEqualOutput<(), U6, U6>;
    type Assert7 = IfLessOrEqualOutput<(), U6, U7>;

    type Assert8 = IfGreaterOutput<(), U7, U4>;

    type Assert9 = IfGreaterOrEqualOutput<(), U7, U4>;
    type Assert10 = IfGreaterOrEqualOutput<(), U7, U7>;

    type Assert11 = IfEqualOutput<(), Z0, Z0>;

    #[test]
    fn control_test() {
        // if constructed
        assert_eq!(3, Assert1::USIZE);

        // if type equivalence
        let _: Assert2 = ();

        // if predicate
        let _: Assert3 = ();

        // if else predicate
        assert!(Assert4::BOOL);

        // if less than
        let _: Assert5 = ();

        // if less than or equal
        let _: Assert6 = ();
        let _: Assert7 = ();

        // if greater than
        let _: Assert8 = ();

        // if greater than or equal
        let _: Assert9 = ();
        let _: Assert10 = ();

        // if equal
        let _: Assert11 = ();
    }
}
