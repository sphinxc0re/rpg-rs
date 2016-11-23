#[macro_export]
macro_rules! vec_enum {
    (
        $(#[$enum_attr:meta])*
        pub enum $name:ident {
            $($variant:ident),*
        }
    ) => {
        $(#[$enum_attr])*
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub fn as_vec() -> Vec<$name> {
                vec![
                    $($name::$variant),*
                ]
            }
        }
    };

    (
        $(#[$enum_attr:meta])*
        enum $name:ident {
            $($variant:ident),*
        }
    ) => {
        $(#[$enum_attr])*
        enum $name {
            $($variant),*
        }

        impl $name {
            pub fn as_vec() -> Vec<$name> {
                vec![
                    $($name::$variant),*
                ]
            }
        }
    };
}
