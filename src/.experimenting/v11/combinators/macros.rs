macro_rules! simple_ctor {
    ($fn_name:ident($($field_name:ident: $field_ty:ty),*)) => {
        #[inline]
        pub fn $fn_name($($field_name: $field_ty),*) -> Self {
            Self {
                $($field_name,)*
            }
        }
    };
}
