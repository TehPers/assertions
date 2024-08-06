use std::{fmt::Display, future::Future};

pub trait SyncCombinator {
    type Target;
    type Result;

    fn to_satisfy<A>(self, assertion: A) -> Self::Result
    where
        A: Assertion<Self::Target, Output = Result<(), ()>>;
}

pub trait AsyncCombinator {
    type Target;
    type Result<A>: Future
    where
        A: Assertion<Self::Target>,
        A::Output: Future<Output = Result<(), ()>>;

    fn to_satisfy<A>(self, assertion: A) -> Self::Result<A>
    where
        A: Assertion<Self::Target>,
        A::Output: Future<Output = Result<(), ()>>;
}

pub trait Assertion<Input>: Display {
    type Output;

    fn assert(self, input: Input) -> Self::Output;
}
