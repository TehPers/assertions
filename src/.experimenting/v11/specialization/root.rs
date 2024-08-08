use std::fmt::{Debug, Display};

use crate::{specialization::Wrapper, ExpectationRoot, SourceLoc};

pub struct __ExpectersOtherTag;

impl __ExpectersOtherTag {
    #[inline]
    pub fn new_root<T>(
        target: T,
        source_info: SourceLoc,
        target_source: &'static str,
    ) -> ExpectationRoot<T> {
        ExpectationRoot::new(target, source_info, target_source)
    }
}

pub trait __ExpectersOtherKind {
    #[inline]
    fn __expecters_kind(&self) -> __ExpectersOtherTag {
        __ExpectersOtherTag
    }
}

impl<T> __ExpectersOtherKind for &Wrapper<&T> {}

pub struct __ExpectersDebugTag;

impl __ExpectersDebugTag {
    #[inline]
    pub fn new_root<T>(
        target: T,
        source_info: SourceLoc,
        target_source: &'static str,
    ) -> ExpectationRoot<T>
    where
        T: Debug,
    {
        // TODO: use `format!("{target:?}")`
        ExpectationRoot::new(target, source_info, target_source)
    }
}

pub trait __ExpectersDebugKind {
    #[inline]
    fn __expecters_kind(&self) -> __ExpectersDebugTag {
        __ExpectersDebugTag
    }
}

impl<T> __ExpectersDebugKind for &&Wrapper<&T> where T: Debug {}

pub struct __ExpectersDisplayTag;

impl __ExpectersDisplayTag {
    #[inline]
    pub fn new_root<T>(
        target: T,
        source_info: SourceLoc,
        target_source: &'static str,
    ) -> ExpectationRoot<T>
    where
        T: Display,
    {
        // TODO: use `format!("{target}")`
        ExpectationRoot::new(target, source_info, target_source)
    }
}

pub trait __ExpectersDisplayKind {
    #[inline]
    fn __expecters_kind(&self) -> __ExpectersDisplayTag {
        __ExpectersDisplayTag
    }
}

impl<T> __ExpectersDisplayKind for &&&Wrapper<&T> where T: Display {}
