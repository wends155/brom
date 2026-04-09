use std::marker::PhantomData;

/// Wrapper for a 1:N foreign key relationship.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link<T> {
    id: i64,
    _marker: PhantomData<T>,
}

impl<T> Link<T> {
    /// Creates a new Link for the given id.
    pub fn new(id: i64) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }

    /// Returns the underlying ID.
    pub fn id(&self) -> i64 {
        self.id
    }
}

/// Metadata marker for an N:M internal junction table.
/// No runtime storage required.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManyToMany<T> {
    _marker: PhantomData<T>,
}

impl<T> ManyToMany<T> {
    /// Creates a new `ManyToMany` metadata marker.
    #[must_use]
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T> Default for ManyToMany<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_roundtrip() {
        struct Dummy;
        let link: Link<Dummy> = Link::new(42);
        assert_eq!(link.id(), 42);
    }

    #[test]
    fn many_to_many_constructors() {
        #[derive(Debug, PartialEq)]
        struct Dummy;
        let m1: ManyToMany<Dummy> = ManyToMany::new();
        let m2: ManyToMany<Dummy> = ManyToMany::default();
        assert_eq!(m1, m2);
    }
}
