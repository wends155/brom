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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_roundtrip() {
        struct Dummy;
        let link: Link<Dummy> = Link::new(42);
        assert_eq!(link.id(), 42);
    }
}
