#[doc(hidden)]
pub struct Wrapper<I, T>(pub I, pub T);

pub mod kinds {
    use std::{
        borrow::Borrow,
        collections::HashMap,
        hash::{BuildHasher, Hash},
        ops::Index,
    };

    use super::Wrapper;

    #[doc(hidden)]
    pub trait __ExpectersForceIndexKind<I> {
        type __ExpectersInput;
        type __ExpectersOutput: ?Sized;

        fn __expecters_try_index(
            self,
        ) -> fn(&Self::__ExpectersInput, I) -> Option<&Self::__ExpectersOutput>;
    }

    impl<I, T> __ExpectersForceIndexKind<I> for &Wrapper<&I, &T>
    where
        T: Index<I>,
    {
        type __ExpectersInput = T;
        type __ExpectersOutput = T::Output;

        fn __expecters_try_index(
            self,
        ) -> fn(&Self::__ExpectersInput, I) -> Option<&Self::__ExpectersOutput> {
            |value, index| Some(&value[index])
        }
    }

    #[doc(hidden)]
    pub trait __ExpectersIteratorKind {
        type __ExpectersInput;
        type __ExpectersOutput;

        fn __expecters_try_index(
            self,
        ) -> fn(Self::__ExpectersInput, usize) -> Option<Self::__ExpectersOutput>;
    }

    impl<T> __ExpectersIteratorKind for &&Wrapper<&usize, &T>
    where
        T: IntoIterator,
    {
        type __ExpectersInput = T;
        type __ExpectersOutput = T::Item;

        fn __expecters_try_index(
            self,
        ) -> fn(Self::__ExpectersInput, usize) -> Option<Self::__ExpectersOutput> {
            |value, index| value.into_iter().nth(index)
        }
    }

    #[doc(hidden)]
    pub trait __ExpectersMapKind<I> {
        type __ExpectersInput;
        type __ExpectersOutput;

        fn __expecters_try_index(
            self,
        ) -> fn(Self::__ExpectersInput, I) -> Option<Self::__ExpectersOutput>;
    }

    impl<K, V, S, Q> __ExpectersMapKind<&Q> for &&&Wrapper<&&Q, &HashMap<K, V, S>>
    where
        K: Eq + Hash + Borrow<Q>,
        V: Clone,
        S: BuildHasher,
        Q: Hash + Eq + ?Sized,
    {
        type __ExpectersInput = HashMap<K, V, S>;
        type __ExpectersOutput = V;

        fn __expecters_try_index(
            self,
        ) -> fn(Self::__ExpectersInput, &Q) -> Option<Self::__ExpectersOutput> {
            |value, index| value.get(index).cloned()
        }
    }
}
