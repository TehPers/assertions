use crate::{
    assertions::{
        pointers::{PointerLike, ToBeNull, ToPointTo},
        AssertionBuilder,
    },
    metadata::Annotated,
};

/// Assertions and modifiers for pointers.
pub trait PointerAssertions<T, M>
where
    T: PointerLike,
{
    /// Asserts that the subject is the null pointer.
    ///
    /// Note that this does not make sense for pointers that cannot be null,
    /// which is most pointer types in Rust.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// expect!(std::ptr::null::<i32>(), to_be_null);
    /// ```
    ///
    /// This assertion fails if the pointer is not null:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// expect!("abc", to_be_null);
    /// ```
    #[inline]
    fn to_be_null(&self) -> ToBeNull {
        ToBeNull::new()
    }

    /// Asserts that the subject points to the same location as a given pointer.
    ///
    /// ```
    /// # use expecters::prelude::*;
    /// let a = Box::new(1);
    /// expect!(&*a, to_point_to(&*a));
    /// ```
    ///
    /// This assertion fails if the pointers do not point to the same location:
    ///
    /// ```should_panic
    /// # use expecters::prelude::*;
    /// let a = Box::new(1);
    /// let b = Box::new(1);
    /// expect!(&*a, to_point_to(&*b));
    /// ```
    ///
    /// Note that comparisons for wide pointers (like `&dyn Trait`) may be
    /// unreliable. See the documentation for [`std::ptr::eq`] for more details.
    #[inline]
    fn to_point_to<U>(&self, other: Annotated<U>) -> ToPointTo<U> {
        ToPointTo::new(other)
    }
}

impl<T, M> PointerAssertions<T, M> for AssertionBuilder<T, M> where T: PointerLike {}
