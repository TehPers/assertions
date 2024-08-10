use std::fmt::{Debug, Display, Formatter};

#[macro_export]
#[doc(hidden)]
macro_rules! annotated {
    ($value:expr) => {{
        #[allow(unused_imports)]
        use $crate::specialization::annotated::*;

        let wrapper = $crate::specialization::__SpecializeWrapper($value);
        (&wrapper)
            .__annotated_kind()
            .annotate(wrapper.0, ::std::stringify!($value))
    }};
}

/// A value annotated with its string representation.
///
/// This holds a string representation of the stored value. The string
/// representation is obtained in the following order of precedence:
///
/// 1. the [`Debug`] representation, otherwise...
/// 2. the [stringified](std::stringify) source code (that was annotated).
///
/// The stringified source code is always available as well, which can be
/// helpful for providing error messages that refer to the actual source code
/// of a value.
///
/// One drawback is that if the annotated value was a variable, the source
/// representation is the name of that variable, which may provide limited
/// information about the actual value that was annotated. This can happen, for
/// example, if a value is an annotated input into an assertion and was
/// generated by the [`expect!`](crate::expect!) macro (which annotates
/// intermediate values inside of closures). In this case, the only way to
/// generate a meaningful string representation of the value is for that value
/// to implement [`Debug`].
///
/// This type makes no guarantees about the string representation of the
/// contained value except for where the representation comes from. Two
/// different compiler versions may result in two different string
/// representations (due to [stringify]'s lack of guarantee). The string
/// representation is *only* intended to be used to augment user-facing
/// messages.
#[derive(Clone, Debug)]
pub struct Annotated<T> {
    value: T,
    string_repr: Option<String>,
    stringified: &'static str,
    kind: AnnotatedKind,
}

impl<T> Annotated<T> {
    #[inline]
    pub(crate) fn from_stringified(value: T, stringified: &'static str) -> Self {
        Self {
            string_repr: None,
            stringified,
            value,
            kind: AnnotatedKind::Stringify,
        }
    }

    /// Gets a reference to the inner value.
    #[inline]
    pub fn inner(&self) -> &T {
        &self.value
    }

    /// Gets a mutable reference to the inner value.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Extracts the inner value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Gets the stringified input's source code.
    #[inline]
    pub fn stringified(&self) -> &'static str {
        self.stringified
    }

    /// Gets the source of the string representation of this value.
    #[inline]
    pub fn kind(&self) -> AnnotatedKind {
        self.kind
    }

    /// Gets the string representation of this value.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.string_repr.as_deref().unwrap_or(self.stringified)
    }
}

impl<T> Annotated<T>
where
    T: Debug,
{
    #[inline]
    pub(crate) fn from_debug(value: T, stringified: &'static str) -> Self {
        Self {
            string_repr: Some(format!("{value:?}")),
            stringified,
            value,
            kind: AnnotatedKind::Debug,
        }
    }
}

impl<T> Display for Annotated<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

/// The source of the string representation for an [`Annotated`] value.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[non_exhaustive]
pub enum AnnotatedKind {
    /// The string representation is the [stringified](stringify) source code.
    Stringify,

    /// The string representation is the [`Debug`] representation of the value.
    Debug,
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::metadata::Annotated;

    use super::AnnotatedKind;

    struct UseStringify<T>(T);

    #[test_case(annotated!(1), AnnotatedKind::Debug, "1", "1"; "debug simple")]
    #[test_case(annotated!(1 + 3), AnnotatedKind::Debug, "1 + 3", "4"; "debug addition")]
    #[test_case(annotated!("test"), AnnotatedKind::Debug, "\"test\"", "\"test\""; "debug string")]
    #[test_case(annotated!(UseStringify(1)), AnnotatedKind::Stringify, "UseStringify(1)", "UseStringify(1)"; "stringify simple")]
    #[test_case(annotated!(UseStringify(1 + 3)), AnnotatedKind::Stringify, "UseStringify(1 + 3)", "UseStringify(1 + 3)"; "stringify addition")]
    fn annotated_macro<T>(
        annotated: Annotated<T>,
        kind: AnnotatedKind,
        stringified: &str,
        as_str: &str,
    ) {
        assert_eq!(annotated.kind(), kind);
        assert_eq!(annotated.stringified(), stringified);
        assert_eq!(annotated.as_str(), as_str);
    }
}
