// TODO:
// - Accept trailting commas for fields

macro_rules! create_dto {
    // matches => Name, required { ..fields }, optional { ..fields }
    (
        $(#[$main_attr:meta])* $main:ident,
        required { $( $(#[$rfield_attr:meta])* $rfield:ident: $mtype:ty ),* $(,)? },
        optional { $( $(#[$ofield_attr:meta])* $ofield:ident: $otype:ty ),* $(,)? }
    ) => {
        paste::paste! {
            $(#[$main_attr])*
            pub struct [<$main Required>] {
                $(
                    $(#[$rfield_attr])*
                    pub $rfield: $mtype,
                )*
            }

            $(#[$main_attr])*
            #[derive(Default)]
            pub struct [<$main Optional>] {
                $(
                    $(#[$ofield_attr])*
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub $ofield: Option<$otype>,
                )*
            }

            $(#[$main_attr])*
            pub struct $main {
                #[serde(flatten)]
                pub required: [<$main Required>],
                #[serde(flatten)]
                pub optional: [<$main Optional>],
            }

            impl_tryfrom_serde_value!($main [<$main Optional>] [<$main Required>]);
        }
    };

    // matches => Name, required { ..fields }
    (
        $(#[$main_attr:meta])* $main:ident,
        required { $( $(#[$rfield_attr:meta])* $rfield:ident: $mtype:ty ),* $(,)? }
    ) => {
        $(#[$main_attr])*
        pub struct $main {
            $(
                $(#[$rfield_attr])*
                pub $rfield: $mtype,
            )*
        }

        impl_tryfrom_serde_value!($main);
    };

    // matches => Name, optional { ..fields }
    (
        $(#[$main_attr:meta])* $main:ident,
        optional { $( $(#[$ofield_attr:meta])* $ofield:ident: $otype:ty ),* $(,)? }
    ) => {
        $(#[$main_attr])*
        #[derive(Default)]
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
