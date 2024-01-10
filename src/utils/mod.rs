mod froms;

// Generic Wrapper struct for newtype pattern, mostly for external type to type From/TryFrom conversions
pub(crate) struct W<T>(pub T);
