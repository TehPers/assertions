use std::{ptr::NonNull, rc::Rc, sync::Arc};

/// A type which points to a memory location.
pub trait PointerLike {
    /// The target being pointed to.
    type Target: ?Sized;

    /// Converts a value into a raw pointer.
    ///
    /// This doesn't accept `&self` to avoid adding `.as_ptr()` methods to a
    /// whole bunch of types.
    fn as_ptr(ptr: &Self) -> *const Self::Target;
}

impl<T> PointerLike for *const T
where
    T: ?Sized,
{
    type Target = T;

    fn as_ptr(ptr: &Self) -> *const Self::Target {
        *ptr
    }
}

impl<T> PointerLike for *mut T
where
    T: ?Sized,
{
    type Target = T;

    fn as_ptr(ptr: &Self) -> *const Self::Target {
        *ptr
    }
}

impl<T> PointerLike for &T
where
    T: ?Sized,
{
    type Target = T;

    fn as_ptr(ptr: &Self) -> *const Self::Target {
        *ptr
    }
}

impl<T> PointerLike for Box<T>
where
    T: ?Sized,
{
    type Target = T;

    fn as_ptr(ptr: &Self) -> *const Self::Target {
        &raw const **ptr
    }
}

impl<T> PointerLike for Rc<T>
where
    T: ?Sized,
{
    type Target = T;

    fn as_ptr(ptr: &Self) -> *const Self::Target {
        Rc::as_ptr(ptr)
    }
}

impl<T> PointerLike for Arc<T>
where
    T: ?Sized,
{
    type Target = T;

    fn as_ptr(ptr: &Self) -> *const Self::Target {
        Arc::as_ptr(ptr)
    }
}

impl<T> PointerLike for NonNull<T>
where
    T: ?Sized,
{
    type Target = T;

    fn as_ptr(ptr: &Self) -> *const Self::Target {
        ptr.as_ptr()
    }
}
