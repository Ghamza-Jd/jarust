macro_rules! make_dto {
    (
        $(#[$main_attr:meta])* $main:ident,
        required { $(#[$rfield_attr:meta])* $rfield:ident: $rtype:ty }
    ) => {
        compile_error!("Overkill, try passing the field directly instead of creating a DTO for it");
    };

    (
        $(#[$main_attr:meta])* $main:ident,
        optional { $(#[$ofield_attr:meta])* $ofield:ident: $otype:ty }
    ) => {
        compile_error!("Overkill, try passing the field directly instead of creating a DTO for it");
    };

    // Arguably overkill, we can pass 2 params to the function instead of constructing a DTO
    // But can be useful for simple named params
    (
        $(#[$main_attr:meta])* $main:ident,
        required { $(#[$rfield_attr:meta])* $rfield:ident: $rtype:ty },
        optional { $(#[$ofield_attr:meta])* $ofield:ident: $otype:ty }
    ) => {
        $(#[$main_attr])*
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, serde::Serialize)]
        pub struct $main {
            $(#[$rfield_attr])*
            pub $rfield: $rtype,
            $(#[$ofield_attr])*
            #[serde(skip_serializing_if = "Option::is_none")]
            pub $ofield: Option<$otype>,
        }

        impl_tryfrom_serde_value!($main);
    };

    (
        $(#[$main_attr:meta])* $main:ident,
        required { $(#[$rfield_attr:meta])* $rfield:ident: $rtype:ty },
        optional { $( $(#[$ofield_attr:meta])* $ofield:ident: $otype:ty ),* $(,)? }
    ) => {
        paste::paste! {
            $(#[$main_attr])*
            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, serde::Serialize)]
            pub struct [<$main Optional>] {
                $(
                    $(#[$ofield_attr])*
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub $ofield: Option<$otype>,
                )*
            }

            $(#[$main_attr])*
            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, serde::Serialize)]
            pub struct $main {
                $(#[$rfield_attr])*
                pub $rfield: $rtype,
                #[serde(flatten)]
                pub optional: [<$main Optional>],
            }

            impl_tryfrom_serde_value!($main [<$main Optional>]);
        }
    };

    (
        $(#[$main_attr:meta])* $main:ident,
        required { $( $(#[$rfield_attr:meta])* $rfield:ident: $rtype:ty ),* $(,)? },
        optional { $(#[$ofield_attr:meta])* $ofield:ident: $otype:ty }
    ) => {
        paste::paste! {
            $(#[$main_attr])*
            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, serde::Serialize)]
            pub struct $main {
                $(
                    $(#[$rfield_attr])*
                    pub $rfield: $rtype,
                )*
                $(#[$ofield_attr])*
                #[serde(skip_serializing_if = "Option::is_none")]
                pub $ofield: Option<$otype>,
            }

            impl_tryfrom_serde_value!($main);
        }
    };

    (
        $(#[$main_attr:meta])* $main:ident,
        required { $( $(#[$rfield_attr:meta])* $rfield:ident: $rtype:ty ),* $(,)? },
        optional { $( $(#[$ofield_attr:meta])* $ofield:ident: $otype:ty ),* $(,)? }
    ) => {
        paste::paste! {
            $(#[$main_attr])*
            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, serde::Serialize)]
            pub struct [<$main Required>] {
                $(
                    $(#[$rfield_attr])*
                    pub $rfield: $rtype,
                )*
            }

            $(#[$main_attr])*
            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, serde::Serialize)]
            pub struct [<$main Optional>] {
                $(
                    $(#[$ofield_attr])*
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub $ofield: Option<$otype>,
                )*
            }

            $(#[$main_attr])*
            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, serde::Serialize)]
            pub struct $main {
                #[serde(flatten)]
                pub required: [<$main Required>],
                #[serde(flatten)]
                pub optional: [<$main Optional>],
            }

            impl_tryfrom_serde_value!($main [<$main Optional>] [<$main Required>]);
        }
    };

    (
        $(#[$main_attr:meta])* $main:ident,
        required { $( $(#[$rfield_attr:meta])* $rfield:ident: $rtype:ty ),* $(,)? }
    ) => {
        $(#[$main_attr])*
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, serde::Serialize)]
        pub struct $main {
            $(
                $(#[$rfield_attr])*
                pub $rfield: $rtype,
            )*
        }

        impl_tryfrom_serde_value!($main);
    };

    (
        $(#[$main_attr:meta])* $main:ident,
        optional { $( $(#[$ofield_attr:meta])* $ofield:ident: $otype:ty ),* $(,)? }
    ) => {
        $(#[$main_attr])*
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, serde::Serialize)]
        pub struct $main {
            $(
                $(#[$ofield_attr])*
                #[serde(skip_serializing_if = "Option::is_none")]
                pub $ofield: Option<$otype>,
            )*
        }

        impl_tryfrom_serde_value!($main);
    };
}
