// Note: This can be a derive macro, but requires to be placed in seperate proc-macro crate, which is an overkill
// for this simple utility.

/// A Utility macro to implement [`TryFrom`] for [`serde_json::Value`].
///
/// Now we can replace [`serde_json::to_value`] with [`TryInto::try_into`] in our code.
///
/// ### Example:
/// ```rust
/// impl_tryfrom_serde_value!(ChangeRoomOptions EditRoomOptions DestroyRoomMsg JoinRoomOptions);
/// ```
macro_rules! impl_tryfrom_serde_value {
    ($($ty:ident)*) => {
        $(
            impl TryFrom<$ty> for serde_json::Value {
                type Error = serde_json::Error;

                fn try_from(val: $ty) -> Result<serde_json::Value, Self::Error> {
                    serde_json::to_value(val)
                }
            }
        )*
    };
}
