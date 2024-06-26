use crate::core::UntypedVal;

/// Utility type used to convert between `reftype` and [`UntypedVal`].
///
/// # Note
///
/// This is used for conversions of [`FuncRef`] and [`ExternRef`].
///
/// [`FuncRef`]: [`crate::FuncRef`]
/// [`ExternRef`]: [`crate::ExternRef`]
pub union Transposer<T: Copy> {
    /// The `reftype` based representation.
    pub reftype: T,
    /// The integer based representation to model pointer types.
    pub value: u64,
}

impl<T: Copy> Transposer<T> {
    /// Creates a `null` [`Transposer`].
    pub fn null() -> Self {
        Self { value: 0 }
    }
}

impl<T: Copy> Transposer<T> {
    /// Creates a new [`Transposer`] from the given `reftype`.
    pub fn new(reftype: T) -> Self {
        Transposer { reftype }
    }
}

impl<T: Copy> From<UntypedVal> for Transposer<T> {
    fn from(untyped: UntypedVal) -> Self {
        Transposer {
            value: u64::from(untyped),
        }
    }
}
