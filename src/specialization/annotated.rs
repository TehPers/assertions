use std::fmt::{Debug, Display};

use crate::metadata::Annotated;

use super::__SpecializeWrapper;

pub struct __AnnotatedNoImplTag;

impl __AnnotatedNoImplTag {
    pub fn __apply<T>(self, _annotated: &mut Annotated<T>) {}
}

pub trait __AnnotatedNoImplKind: Sized {
    #[inline]
    fn __kind(self) -> __AnnotatedNoImplTag {
        __AnnotatedNoImplTag
    }
}

impl<T> __AnnotatedNoImplKind for &__SpecializeWrapper<T> where T: ?Sized {}

pub struct __AnnotatedDebugTag;

impl __AnnotatedDebugTag {
    #[inline]
    pub fn __apply<T>(self, annotated: &mut Annotated<T>)
    where
        T: Debug,
    {
        annotated.mark_debug();
    }
}

pub trait __AnnotatedDebugKind: Sized {
    #[inline]
    fn __kind(self) -> __AnnotatedDebugTag {
        __AnnotatedDebugTag
    }
}

impl<T> __AnnotatedDebugKind for __SpecializeWrapper<(T, dyn Debug)> where T: Debug {}

pub struct __AnnotatedDisplayTag;

impl __AnnotatedDisplayTag {
    #[inline]
    pub fn __apply<T>(self, annotated: &mut Annotated<T>)
    where
        T: Display,
    {
        annotated.mark_display();
    }
}

pub trait __AnnotatedDisplayKind: Sized {
    #[inline]
    fn __kind(self) -> __AnnotatedDisplayTag {
        __AnnotatedDisplayTag
    }
}

impl<T> __AnnotatedDisplayKind for __SpecializeWrapper<(T, dyn Display)> where T: Display {}
