
macro_rules! fields_comparable {
    ([$($field:ident),*], $name:ident) => {
        impl $name {
            pub fn compare(s0: $name, s1: $name) -> bool {
                $(if(s0.field != s1.field) { false })*
                true
            }
        }
    }
}