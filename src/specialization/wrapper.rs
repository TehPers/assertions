use std::marker::PhantomData;

pub struct __SpecializeWrapper<T: ?Sized>(PhantomData<*const T>);

impl<T> __SpecializeWrapper<T> {
    #[inline]
    pub(crate) fn new() -> Self {
        __SpecializeWrapper(PhantomData)
    }

    #[inline]
    pub fn __for_trait<Trait: ?Sized>(&self) -> __SpecializeWrapper<(T, Trait)> {
        __SpecializeWrapper(PhantomData)
    }
}
