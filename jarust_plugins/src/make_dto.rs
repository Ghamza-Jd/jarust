macro_rules! make_dto {
    // matches => Name, required { }, optional { }
    (
        $(#[$main_attr:meta])* $main:ident,
        required { $( $(#[$rfield_attr:meta])* $rfield:ident: $mtype:ty ),* $(,)? },
        optional { $( $(#[$ofield_attr:meta])* $ofield:ident: $otype:ty ),* $(,)? }
    ) => {
        paste::paste! {
            $(#[$main_attr])*
            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, serde::Serialize)]
            pub struct [<$main Required>] {
                $(
                    $(#[$rfield_attr])*
                    pub $rfield: $mtype,
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

    // matches => Name, required { }
    (
        $(#[$main_attr:meta])* $main:ident,
        required { $( $(#[$rfield_attr:meta])* $rfield:ident: $mtype:ty ),* $(,)? }
    ) => {
        $(#[$main_attr])*
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, serde::Serialize)]
        pub struct $main {
            $(
                $(#[$rfield_attr])*
                pub $rfield: $mtype,
            )*
        }

        impl_tryfrom_serde_value!($main);
    };

    // matches => Name, optional { }
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
