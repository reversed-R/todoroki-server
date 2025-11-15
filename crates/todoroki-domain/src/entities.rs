pub mod client;
pub mod label;
pub mod todo;
pub mod user;
pub mod user_auth;

#[macro_export]
macro_rules! value_object {
    ($name:ident($inner_typ:ty)) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name($inner_typ);

        impl $name {
            pub fn new(value: $inner_typ) -> Self {
                Self(value)
            }

            pub fn value(self) -> $inner_typ {
                self.0
            }
        }
    };
}
