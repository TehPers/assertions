use std::fmt::Debug;

use crate::metadata::Annotated;

use super::__SpecializeWrapper;

pub struct __AnnotatedStringifyTag;

impl __AnnotatedStringifyTag {
    pub fn annotate<T>(self, value: T, stringified: &'static str) -> Annotated<T> {
        Annotated::from_stringified(value, stringified)
    }
}
pub trait __AnnotatedStringifyKind {
    #[inline]
    fn __annotated_kind(&self) -> __AnnotatedStringifyTag {
        __AnnotatedStringifyTag
    }
}

impl<T> __AnnotatedStringifyKind for &__SpecializeWrapper<T> {}

pub struct __AnnotatedDebugTag;

impl __AnnotatedDebugTag {
    pub fn annotate<T>(self, value: T, stringified: &'static str) -> Annotated<T>
    where
        T: Debug,
    {
        Annotated::from_debug(value, stringified)
    }
}

pub trait __AnnotatedDebugKind {
    #[inline]
    fn __annotated_kind(&self) -> __AnnotatedDebugTag {
        __AnnotatedDebugTag
    }
}

impl<T> __AnnotatedDebugKind for __SpecializeWrapper<T> where T: Debug {}
