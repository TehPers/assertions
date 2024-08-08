mod all;
mod assertion;
mod combinator;
mod error;
mod not;

pub use all::*;
pub use assertion::*;
pub use combinator::*;
pub use error::*;
pub use not::*;

use std::fmt::{self, Display, Formatter};

/////////

#[derive(Clone, Debug)]
pub struct AssertionRoot<T> {
    target: T,
}

impl<T, A> AssertionCombinator<A> for AssertionRoot<T>
where
    A: Assertion<T>,
{
    type NextInput = T;
    type Assertion = RootAssertion<T, A>;

    fn build(self, assertion: A) -> Self::Assertion {
        RootAssertion {
            target: self.target,
            next: assertion,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RootAssertion<T, Next> {
    target: T,
    next: Next,
}

impl<T, Next> Display for RootAssertion<T, Next>
where
    Next: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.next)
    }
}

impl<T, Next> Assertion<()> for RootAssertion<T, Next>
where
    Next: Assertion<T>,
{
    type Output = Next::Output;

    fn assert(self, _target: ()) -> Self::Output {
        self.next.assert(self.target)
    }
}

fn a() {
    let root = AssertionRoot { target: [1, 2, 3] };
    let assertion = root
        .all()
        .not()
        .build(SimpleAssertion::new("not zero", |value| {
            if value != 0 {
                Ok(())
            } else {
                Err(AssertionError::default())
            }
        }));

    let _result = assertion.assert(());
}

// #[derive(Clone, Debug)]
// pub struct MapAssertion<Next, I, O> {
//     next: Next,
//     f: fn(I) -> O,
// }

// impl<Next, I, O> Display for MapAssertion<Next, I, O>
// where
//     Next: Display,
// {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         todo!()
//     }
// }

// impl<Next, I, O> Assertion for MapAssertion<Next, I, O>
// where
//     Next: Assertion<Input = O>,
// {
//     type Input = I;
//     type Output = Next::Output;

//     fn assert(self, target: Self::Input) -> Self::Output {
//         self.next.assert((self.f)(target))
//     }
// }

// #[derive(Clone, Debug)]
// pub struct WhenReadyAssertion<I, Next> {
//     next: Next,
//     marker: PhantomData<fn(I)>,
// }

// impl<I, Next> Display for WhenReadyAssertion<I, Next>
// where
//     Next: Display,
// {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         todo!()
//     }
// }

// impl<I, Next> Assertion for WhenReadyAssertion<I, Next>
// where
//     I: Future + Send + 'static,
//     Next: Assertion<Input = I::Output> + Send + 'static,
// {
//     type Input = I;
//     type Output = Pin<Box<dyn Future<Output = Next::Output> + Send>>;

//     fn assert(self, target: Self::Input) -> Self::Output {
//         Box::pin(async move { self.next.assert(target.await) })
//     }
// }

// async fn foo() {
//     let assertion = RootAssertion {
//         target: ready([1, 2, 3]),
//         next: WhenReadyAssertion {
//             marker: PhantomData,
//             next: AllAssertion {
//                 marker: PhantomData,
//                 next: SimpleAssertion::new("non-zero", |value| {
//                     if value == 0 {
//                         Err(AssertionFailure::default())
//                     } else {
//                         Ok(())
//                     }
//                 }),
//             },
//         },
//     };
//     let result = assertion.assert(()).await;

//     let assertion = RootAssertion {
//         target: [1, 2, 3],
//         next: AllAssertion {
//             marker: PhantomData,
//             next: SimpleAssertion::new("non-zero", |value| {
//                 if value == 0 {
//                     Err(AssertionFailure::default())
//                 } else {
//                     Ok(())
//                 }
//             }),
//         },
//     };
//     let result = assertion.assert(());

//     let combinator = AllCombinator {
//         marker: PhantomData,
//         prev: AssertionRoot { target: [1, 2, 3] },
//     };
//     let assertion = combinator.build(SimpleAssertion::new("non-zero", |value| {
//         if value == 0 {
//             Err(AssertionFailure::default())
//         } else {
//             Ok(())
//         }
//     }));
//     let result = assertion.assert(());
// }
