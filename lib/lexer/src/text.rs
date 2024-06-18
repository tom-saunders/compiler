mod internal;

pub mod char_escape;
pub mod hex_escape;
pub mod oct_escape;
pub mod universal_char;

pub use internal::text_state_impl_i16;
pub use internal::text_state_impl_i32;
pub use internal::text_state_impl_i8;
pub use internal::TextState;
