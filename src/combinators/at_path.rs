use std::fmt::Display;

use crate::Assertable;

/// Wraps an [`Assertable`] and applies an assertion to a sub-path within the
/// target value.
pub struct AtPath<Inner, T>
where
    Inner: Assertable,
{
    inner: Inner,
    traversal: Traversal<Inner::Target, T>,
}

impl<Inner, T> AtPath<Inner, T>
where
    Inner: Assertable,
{
    /// Creates a new combinator which wraps an inner [`Assertable`].
    pub fn new(inner: Inner, traversal: Traversal<Inner::Target, T>) -> Self {
        Self { inner, traversal }
    }
}

impl<Inner, T> Assertable for AtPath<Inner, T>
where
    Inner: Assertable,
{
    type Target = T;
    type Result = Inner::Result;

    fn to_satisfy<F>(self, expectation: impl Display, mut f: F) -> Self::Result
    where
        F: FnMut(Self::Target) -> bool,
    {
        self.inner.to_satisfy(
            format_args!(
                "for the value at path '{}', {}",
                self.traversal.path, expectation
            ),
            |outer| (self.traversal.f)(outer).is_some_and(&mut f),
        )
    }
}

/// Creates a new [`Traversal`] that navigates to a specific path within a target
/// value.
///
/// The path may contain any number of segments, each prefixed by a period. For
/// example, the path `.foo.bar.baz` would navigate from the value's `foo` field
/// down all the way to the `baz` field.
///
/// In addition to navigating to deeply nested fields, this macro also handles
/// fallible values along the way. By putting a question mark after a segment,
/// the traversal will automatically handle the fallible path.
///
/// In addition to the above, more types of traversals are supported. The
/// following is a list of different kinds of traversals that can be performed:
///
/// - Field access: `.field`
/// - Method call: `.method(args...)`
/// - Indexing: `[index]` (see below)
/// - Unwrapping: `?`
///   - This is primarily used to naviate to the inner value of a `Result` or
///     `Option`, but can be used for other types as well. This converts the
///     value to an iterator and takes the first element, if it exists.
/// - Function call (if not a method call): `(args...)`
/// - Pattern matching: `pattern => path`
///   - This is only supported at the top-level, meaning the full path needs to
///     start with the pattern, followed by a fat arrow, followed by an ident,
///     followed by any of the other traversals. For example, this is a valid
///     path: `Some(n) => n?.0`.
///
/// By chaining these traversals together, you can navigate to just about any
/// deeply nested path within a target value. For example, a traversal within
/// a list of lists of tuples could look like `.field[1][4].3?[0].2`.
///
/// ```
/// # use expecters::prelude::*;
/// struct Foo {
///    bar: Option<Bar>,
/// }
///
/// struct Bar(Vec<i32>);
///
/// let value = Some(Foo {
///     bar: Some(Bar(vec![1, 2])),
/// });
/// expect!(value).at_path(path!(Some(foo) => foo.bar?.0[1])).to_equal(2);
/// ```
///
/// ## Indexing
///
/// Indexing is a unique case during path traversal which uses a form of
/// specialization to handle different kinds of types. As a base case, indexing
/// is supported for all types which can be indexed with a particular key, and
/// where the returned value implements [`Clone`]. This base case panics if the
/// index is out of bounds.
///
/// However, while the base case should cover many common cases, there are some
/// specializations which can "safely" index into a value and avoid the default
/// panic behavior. This lets `expecters` create its own custom errors for
/// indexing failures, which can be more informative than a panic.
///
/// In the order they are checked, the following specializations are supported:
///
/// 1. `HashMap<K, V, S> where V: Clone` - You can index this type like normal,
///    and a clone of the value will be returned if it exists.
/// 2. `T: IntoIterator` - If the index is a `usize`, the value will be indexed
///    by converting it to an iterator and taking the element at that index.
///    This relaxes the requirement that the value must be `Clone` since it
///    consumes the container directly.
/// 3. `T: Index<I> where T::Output: Clone` (base case) - This indexes the
///    container like normal, but uses the default panicking behavior if the
///    index is out of bounds.
///
/// If the [`Clone`] bound is too restrictive, consider using one of the other
/// combinators to navigate into the value (like
/// [`map`](crate::Assertable::map)).
#[macro_export]
macro_rules! path {
    ($($path:tt)*) => {
        $crate::combinators::Traversal::new(
            ::std::stringify!($($path)*),
            Box::new(|value| $crate::path_inner!(@traverse value, $($path)*)),
        )
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! path_inner {
    // Base case
    (@traverse $value:expr,) => {
        ::core::option::Option::Some($value)
    };

    // Pattern
    (@traverse $value:expr, $pattern:pat => $path:ident $($rest:tt)*) => {
        match $value {
            $pattern => $crate::path_inner!(@traverse $path, $($rest)*),

            #[allow(unreachable_patterns)]
            _ => ::core::option::Option::None,
        }
    };

    // Method call
    (@traverse $value:expr, .$path:ident ($($args:tt)*) $($rest:tt)*) => {
        $crate::path_inner!(@traverse $value.$path($($args)*), $($rest)*)
    };

    // Simple path traversal
    (@traverse $value:expr, .$path:tt $($rest:tt)*) => {
        $crate::path_inner!(@traverse $value.$path, $($rest)*)
    };

    // Fallible traversal
    (@traverse $value:expr, ? $($rest:tt)*) => {{
        let mut iterator = ::core::iter::IntoIterator::into_iter($value);
        let value = ::core::iter::Iterator::next(&mut iterator)?;
        $crate::path_inner!(@traverse value, $($rest)*)
    }};

    // Indexing traversal
    (@traverse $value:expr, [$index:expr] $($rest:tt)*) => {{
        #[allow(unused_imports)]
        use $crate::specialization::at_path::kinds::*;

        let index = $index;
        let value = $value;
        let wrapper = $crate::specialization::at_path::Wrapper(&index, &value);
        let getter = (&&&wrapper).__expecters_try_index();
        let value = getter(value, index)?;
        $crate::path_inner!(@traverse value, $($rest)*)
    }};

    // Function call
    (@traverse $value:expr, ($($args:tt)*) $($rest:tt)*) => {{
        let value = $value($($args)*);
        $crate::path_inner!(@traverse value, $($rest)*)
    }};
}

/// A traversal to a specific path within a target value.
///
/// This type is created using the [`path!`] macro.
pub struct Traversal<T, U> {
    path: &'static str,
    f: Box<dyn Fn(T) -> Option<U>>,
}

impl<T, U> Traversal<T, U> {
    #[doc(hidden)]
    pub fn new(path: &'static str, f: Box<dyn Fn(T) -> Option<U>>) -> Self {
        Self { path, f }
    }

    /// Applies the traversal to a target value. If the traversal fails at any
    /// point, this method will return `None`.
    #[inline]
    pub fn apply(self, value: T) -> Option<U> {
        (self.f)(value)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::expect;

    use super::*;

    #[derive(Clone, Default)]
    struct Foo {
        bar: Bar,
        opt_bar: Option<Bar>,
    }

    #[derive(Clone, Default)]
    struct Bar {
        baz: i32,
        opt_baz: Option<i32>,
    }

    struct A<T>(pub T);

    #[test]
    fn traversal() {
        expect!(Foo::default()).at_path(path!(.bar.baz)).to_equal(0);
        expect!((1, 2, 3)).at_path(path!(.2)).to_equal(3);
    }

    #[test]
    fn fallible() {
        expect!(Foo::default())
            .not()
            .at_path(path!(.opt_bar?.opt_baz?))
            .to_equal(0);
    }

    #[test]
    fn indexing() {
        expect!([A(1), A(2), A(3)])
            .at_path(path!([1].0))
            .to_equal(2);
        expect!([A(A(1))]).at_path(path!([0].0 .0)).to_equal(1);
        expect!({
            let mut map = HashMap::new();
            map.insert("a".to_string(), 1);
            map.insert("b".to_string(), 2);
            map
        })
        .at_path(path!(["b"]))
        .to_equal(2);

        expect!(vec![1, 2, 3])
            .not()
            .at_path(path!([3]))
            .to_be_greater_than(0);
        expect!([1, 2, 3])
            .not()
            .at_path(path!([3]))
            .to_be_greater_than(0);
        expect!({
            let mut map = HashMap::new();
            map.insert("a".to_string(), 1);
            map.insert("b".to_string(), 2);
            map
        })
        .not()
        .at_path(path!(["c"]))
        .to_equal(2);
    }

    #[test]
    fn patterns() {
        expect!(Some(1))
            .at_path(path!(Some(n) => n.to_string()))
            .to_equal("1");
        expect!(A(1)).at_path(path!(A(n) => n)).to_equal(1);
    }
}
